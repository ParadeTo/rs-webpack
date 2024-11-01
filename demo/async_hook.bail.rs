use async_trait::async_trait;

#[async_trait]
pub trait Interceptor<H: Hook> {
    async fn call(&self, _hook: &H) -> Result<Vec<<H as Hook>::Tap>, ()> {
        unreachable!("Interceptor::call should only used in async hook")
    }

    fn call_blocking(&self, _hook: &H) -> Result<Vec<<H as Hook>::Tap>, ()> {
        unreachable!("Interceptor::call_blocking should only used in sync hook")
    }
}

pub trait Hook {
    type Tap;

    fn used_stages(&self) -> i32;

    fn intercept(&mut self, interceptor: impl Interceptor<Self> + Send + Sync + 'static)
    where
        Self: Sized;
}

pub struct Compilation {
    pub id: u32,
}

pub struct Source {
    pub content: String,
}

#[derive(Default)]
pub struct MyRenderPlugin {}

impl MyRenderPlugin {
    #[allow(clippy::ptr_arg)]
    async fn render(
        &self,
        compilation: &Compilation,
        source: &mut Source,
    ) -> Result<Option<bool>, ()> {
        source.content += "plugin.render";
        source.content += &compilation.id.to_string();
        Ok(Some(true))
    }
}

#[async_trait]
impl Render for MyRenderPlugin {
    async fn run(
        &self,
        compilation: &Compilation,
        source: &mut Source,
    ) -> Result<Option<bool>, ()> {
        MyRenderPlugin::render(&self, compilation, source).await
    }
}

#[async_trait]
pub trait Render {
    async fn run(
        &self,
        compilation: &Compilation,
        source: &mut Source,
    ) -> Result<std::option::Option<bool>, ()>;
    fn stage(&self) -> i32 {
        0
    }
}

// Hook
pub struct RenderHook {
    taps: Vec<Box<dyn Render + Send + Sync>>,
    interceptors: Vec<Box<dyn Interceptor<Self> + Send + Sync>>,
}
impl Hook for RenderHook {
    type Tap = Box<dyn Render + Send + Sync>;
    fn used_stages(&self) -> i32 {
        1
    }
    fn intercept(&mut self, interceptor: impl Interceptor<Self> + Send + Sync + 'static) {
        self.interceptors.push(Box::new(interceptor));
    }
}
impl std::fmt::Debug for RenderHook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RenderHook")
    }
}
impl Default for RenderHook {
    fn default() -> Self {
        Self {
            taps: Default::default(),
            interceptors: Default::default(),
        }
    }
}
impl RenderHook {
    pub async fn call(
        &self,
        compilation: &Compilation,
        source: &mut Source,
    ) -> Result<std::option::Option<bool>, ()> {
        let mut additional_taps = std::vec::Vec::new();
        for interceptor in self.interceptors.iter() {
            additional_taps.extend(interceptor.call(self).await?);
        }
        let mut all_taps = std::vec::Vec::new();
        all_taps.extend(&self.taps);
        all_taps.extend(&additional_taps);
        all_taps.sort_by_key(|hook| hook.stage());
        for tap in all_taps {
            if let Some(res) = tap.run(compilation, source).await? {
                return Ok(Some(res));
            }
        }
        Ok(None)
    }
    pub fn tap(&mut self, tap: impl Render + Send + Sync + 'static) {
        self.taps.push(Box::new(tap));
    }
}

// use render_hook::{Compilation, MyRenderPlugin, RenderHook, Source};

// mod render_hook;
// // mod sync_hook;

// #[tokio::main]
// async fn main() -> Result<(), ()> {
//     let mut compilation = Compilation { id: 0 };
//     let mut source = Source {
//         content: String::new(),
//     };
//     let mut render_hook = RenderHook::default();
//     let plugin = MyRenderPlugin::default();
//     render_hook.tap(plugin);
//     let result = render_hook.call(&compilation, &mut source).await?;
//     println!("{}", source.content);
//     Ok(())
// }
