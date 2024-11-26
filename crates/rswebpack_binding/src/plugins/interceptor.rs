use napi::bindgen_prelude::{FromNapiValue, ToNapiValue};
use napi::{Env, JsFunction, NapiRaw};
use rswebpack_core::compiler::Compiler;
use rswebpack_core::hooks::{BeforeRun, BeforeRunHook, BeforeRunSync, BeforeRunSyncHook};
use rswebpack_hook::__macro_helper::async_trait;
use rswebpack_hook::{Hook, Interceptor};
use rswebpack_napi::threadsafe_function::ThreadsafeFunction;
use std::borrow::Cow;
use std::sync::{Arc, RwLock};

#[napi(object)]
pub struct JsTap {
  pub function: JsFunction,
  pub stage: i32,
}
pub struct ThreadsafeJsTap<T: 'static, R> {
  pub function: ThreadsafeFunction<T, R>,
  pub stage: i32,
}

impl<T: 'static, R> Clone for ThreadsafeJsTap<T, R> {
  fn clone(&self) -> Self {
    Self {
      function: self.function.clone(),
      stage: self.stage,
    }
  }
}

impl<T: 'static + ToNapiValue, R> ThreadsafeJsTap<T, R> {
  pub fn from_js_tap(js_tap: JsTap, env: Env) -> napi::Result<Self> {
    let function =
      unsafe { ThreadsafeFunction::from_napi_value(env.raw(), js_tap.function.raw()) }?;
    Ok(Self {
      function,
      stage: js_tap.stage,
    })
  }
}

impl<T: 'static + ToNapiValue, R> FromNapiValue for ThreadsafeJsTap<T, R> {
  unsafe fn from_napi_value(
    env: napi::sys::napi_env,
    napi_val: napi::sys::napi_value,
  ) -> napi::Result<Self> {
    let t = JsTap::from_napi_value(env, napi_val)?;
    ThreadsafeJsTap::from_js_tap(t, Env::from_raw(env))
  }
}

type RegisterFunctionOutput<T, R> = Vec<ThreadsafeJsTap<T, R>>;
type RegisterFunction<T, R> = ThreadsafeFunction<Vec<i32>, RegisterFunctionOutput<T, R>>;

struct RegisterJsTapsInner<T: 'static, R> {
  register: RegisterFunction<T, R>,
  cache: RegisterJsTapsCache<T, R>,
  non_skippable_registers: Option<NonSkippableRegisters>,
}

impl<T: 'static, R> Clone for RegisterJsTapsInner<T, R> {
  fn clone(&self) -> Self {
    Self {
      register: self.register.clone(),
      cache: self.cache.clone(),
      non_skippable_registers: self.non_skippable_registers.clone(),
    }
  }
}

enum RegisterJsTapsCache<T: 'static, R> {
  NoCache,
  Cache(Arc<tokio::sync::OnceCell<RegisterFunctionOutput<T, R>>>),
  SyncCache(Arc<once_cell::sync::OnceCell<RegisterFunctionOutput<T, R>>>),
}

impl<T: 'static, R> Clone for RegisterJsTapsCache<T, R> {
  fn clone(&self) -> Self {
    match self {
      Self::NoCache => Self::NoCache,
      Self::Cache(c) => Self::Cache(c.clone()),
      Self::SyncCache(c) => Self::SyncCache(c.clone()),
    }
  }
}

impl<T: 'static, R> RegisterJsTapsCache<T, R> {
  pub fn new(cache: bool, sync: bool) -> Self {
    if cache {
      if sync {
        Self::SyncCache(Default::default())
      } else {
        Self::Cache(Default::default())
      }
    } else {
      Self::NoCache
    }
  }
}

impl<T: 'static + ToNapiValue, R: 'static> RegisterJsTapsInner<T, R> {
  pub fn new(
    register: RegisterFunction<T, R>,
    non_skippable_registers: Option<NonSkippableRegisters>,
    cache: bool,
    sync: bool,
  ) -> Self {
    Self {
      register,
      cache: RegisterJsTapsCache::new(cache, sync),
      non_skippable_registers,
    }
  }

  pub async fn call_register(
    &self,
    hook: &impl Hook,
  ) -> rswebpack_error::Result<Cow<RegisterFunctionOutput<T, R>>> {
    if let RegisterJsTapsCache::Cache(cache) = &self.cache {
      let js_taps = cache
        .get_or_try_init(|| self.call_register_impl(hook))
        .await?;
      Ok(Cow::Borrowed(js_taps))
    } else {
      let js_taps = self.call_register_impl(hook).await?;
      Ok(Cow::Owned(js_taps))
    }
  }

  async fn call_register_impl(
    &self,
    hook: &impl Hook,
  ) -> rswebpack_error::Result<RegisterFunctionOutput<T, R>> {
    let mut used_stages = Vec::from_iter(hook.used_stages());
    used_stages.sort();
    println!("call_register_impl");
    self.register.call_with_sync(used_stages).await
  }

  pub fn call_register_blocking(
    &self,
    hook: &impl Hook,
  ) -> rswebpack_error::Result<Cow<RegisterFunctionOutput<T, R>>> {
    if let RegisterJsTapsCache::SyncCache(cache) = &self.cache {
      let js_taps = cache.get_or_try_init(|| self.call_register_blocking_impl(hook))?;
      Ok(Cow::Borrowed(js_taps))
    } else {
      let js_taps = self.call_register_blocking_impl(hook)?;

      Ok(Cow::Owned(js_taps))
    }
  }

  fn call_register_blocking_impl(
    &self,
    hook: &impl Hook,
  ) -> rswebpack_error::Result<RegisterFunctionOutput<T, R>> {
    let mut used_stages = Vec::from_iter(hook.used_stages());
    // println!("call_register_blocking_impl {:?}", used_stages);
    used_stages.sort();
    self.register.blocking_call_with_sync(used_stages)
  }
}

#[napi]
#[derive(Debug, PartialEq, Eq)]
pub enum RegisterJsTapKind {
  BeforeRun,
  BeforeRunSync
}

#[derive(Default, Clone, Debug)]
pub struct NonSkippableRegisters(Arc<RwLock<Vec<RegisterJsTapKind>>>);

impl NonSkippableRegisters {
  pub fn set_non_skippable_registers(&self, kinds: Vec<RegisterJsTapKind>) {
    let mut ks = self.0.write().expect("failed to write lock");
    *ks = kinds;
  }

  pub fn is_non_skippable(&self, kind: &RegisterJsTapKind) -> bool {
    self.0.read().expect("should lock").contains(kind)
  }
}

macro_rules! define_register {
  ($name:ident, tap = $tap_name:ident<$arg:ty, $ret:ty> @ $tap_hook:ty, cache = $cache:literal, sync = $sync:tt, kind = $kind:expr, skip = $skip:tt,) => {
    define_register!(@BASE $name, $tap_name<$arg, $ret>, $cache, $sync);
    define_register!(@SKIP $name, $arg, $ret, $cache, $sync, $skip);
    define_register!(@INTERCEPTOR $name, $tap_name, $tap_hook, $cache, $kind, $sync);
  };
  (@BASE $name:ident, $tap_name:ident<$arg:ty, $ret:ty>, $cache:literal, $sync:literal) => {
    #[derive(Clone)]
    pub struct $name {
      inner: RegisterJsTapsInner<$arg, $ret>,
    }

    #[derive(Clone)]
    struct $tap_name {
      function: ThreadsafeFunction<$arg, $ret>,
      stage: i32,
    }

    impl $tap_name {
      pub fn new(tap: ThreadsafeJsTap<$arg, $ret>) -> Self {
        Self {
          function: tap.function,
          stage: tap.stage,
        }
      }
    }
  };
  (@SKIP $name:ident, $arg:ty, $ret:ty, $cache:literal, $sync:literal, $skip:literal) => {
    impl $name {
      pub fn new(register: RegisterFunction<$arg, $ret>, non_skippable_registers: NonSkippableRegisters) -> Self {
        Self {
          inner: RegisterJsTapsInner::new(register, $skip.then_some(non_skippable_registers), $cache, $sync),
        }
      }
    }
  };
  (@INTERCEPTOR $name:ident, $tap_name:ident, $tap_hook:ty, $cache:literal, $kind:expr, false) => {
    #[async_trait]
    impl Interceptor<$tap_hook> for $name {
      async fn call(
        &self,
        hook: &$tap_hook,
      ) -> rswebpack_error::Result<Vec<<$tap_hook as Hook>::Tap>> {
        if let Some(non_skippable_registers) = &self.inner.non_skippable_registers {
          if !non_skippable_registers.is_non_skippable(&$kind) {
            return Ok(Vec::new());
          }
        }
        let js_taps = self.inner.call_register(hook).await?;
        let js_taps = js_taps
          .iter()
          .map(|t| Box::new($tap_name::new(t.clone())) as <$tap_hook as Hook>::Tap)
          .collect();
        Ok(js_taps)
      }
    }
  };
  (@INTERCEPTOR $name:ident, $tap_name:ident, $tap_hook:ty, $cache:literal, $kind:expr, true) => {
    impl Interceptor<$tap_hook> for $name {
      fn call_blocking(
        &self,
        hook: &$tap_hook,
      ) -> rswebpack_error::Result<Vec<<$tap_hook as Hook>::Tap>> {

        if let Some(non_skippable_registers) = &self.inner.non_skippable_registers {
          if !non_skippable_registers.is_non_skippable(&$kind) {
            return Ok(Vec::new());
          }
        }
        let js_taps = self.inner.call_register_blocking(hook)?;
        println!("call_blocking {:?}", js_taps.len());
        let js_taps = js_taps
          .iter()
          .map(|t| Box::new($tap_name::new(t.clone())) as <$tap_hook as Hook>::Tap)
          .collect();
        Ok(js_taps)
      }
    }
  };
}

#[derive(Clone)]
#[napi(object, object_to_js = false)]
pub struct RegisterJsTaps {
  #[napi(
    ts_type = "(stages: Array<number>) => Array<{ function: ((arg: string) => void); stage: number; }>"
  )]
  pub register_before_run_taps: RegisterFunction<String, ()>,
  #[napi(
    ts_type = "(stages: Array<number>) => Array<{ function: ((arg: string) => void); stage: number; }>"
  )]
  pub register_before_run_sync_taps: RegisterFunction<String, ()>,
}

define_register!(
    RegisterBeforeRunTaps,
    tap = BeforeRunTap<String, ()> @ BeforeRunHook,
    cache = false,
    sync = false,
    kind = RegisterJsTapKind::BeforeRun,
    skip = true,
);

define_register!(
    RegisterBeforeRunSyncTaps,
    tap = BeforeRunSyncTap<String, ()> @ BeforeRunSyncHook,
    cache = false,
    sync = true,
    kind = RegisterJsTapKind::BeforeRunSync,
    skip = true,
);

#[async_trait]
impl BeforeRun for BeforeRunTap {
  async fn run(&self, compiler: &mut Compiler) -> rswebpack_error::Result<()> {
    self.function.call_with_sync(compiler.root.clone()).await
  }

  fn stage(&self) -> i32 {
    self.stage
  }
}

impl BeforeRunSync for BeforeRunSyncTap {
  fn run(&self, compiler: &mut Compiler) -> rswebpack_error::Result<()> {
    self.function.blocking_call_with_sync(compiler.root.clone())
  }

  fn stage(&self) -> i32 {
    self.stage
  }
}
