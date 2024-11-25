use crate::plugins::interceptor::{NonSkippableRegisters, RegisterBeforeRunTaps, RegisterJsTapKind, RegisterJsTaps};
use napi::Env;
use napi::Result;
use rswebpack_core::compiler::Compiler;
use rswebpack_core::hooks::BeforeRun;
use rswebpack_core::plugin::{ApplyContext, Plugin, PluginContext};
use rswebpack_hook::__macro_helper::async_trait;
use rswebpack_hook::{plugin, plugin_hook, Hook};
use std::fmt;

pub mod interceptor;

#[plugin]
#[derive(Clone)]
pub struct JsHooksAdapterPlugin {
  register_before_run_taps: RegisterBeforeRunTaps,
  non_skippable_registers: NonSkippableRegisters
}

impl fmt::Debug for JsHooksAdapterPlugin {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "rspack_plugin_js_hooks_adapter")
  }
}

// #[plugin_hook(BeforeRun for JsHooksAdapterPlugin)]
// fn before_run(&self, compiler: &mut Compiler) -> rswebpack_error::Result<()> {
//   println!("Root is {}", compiler.root);
//   Ok(())
// }

#[async_trait]
impl Plugin for JsHooksAdapterPlugin {
  fn name(&self) -> &'static str {
    "rspack.JsHooksAdapterPlugin"
  }

  fn apply(&self, _ctx: PluginContext<&mut ApplyContext>) -> rswebpack_error::Result<()> {
    println!("JsHooksAdapterPlugin apply called");
    _ctx
      .context
      .compiler_hooks
      .before_run
      .intercept(self.register_before_run_taps.clone());
    Ok(())
  }
}

impl JsHooksAdapterPlugin {
  pub fn from_js_hooks(register_js_taps: RegisterJsTaps) -> Result<Self> {
    let non_skippable_registers = NonSkippableRegisters::default();
    Ok(JsHooksAdapterPlugin {
      inner: JsHooksAdapterPluginInner {
        register_before_run_taps: RegisterBeforeRunTaps::new(
          register_js_taps.register_before_run_taps,
          non_skippable_registers.clone(),
        ),
        non_skippable_registers,
      }
      .into(),
    })
  }

  pub fn set_non_skippable_registers(&self, kinds: Vec<RegisterJsTapKind>) {
    self
        .non_skippable_registers
        .set_non_skippable_registers(kinds);
  }
}
