#![deny(clippy::all)]

use crate::plugins::interceptor::{RegisterBeforeRunTaps, RegisterJsTapKind, RegisterJsTaps};
use crate::plugins::JsHooksAdapterPlugin;
use napi::Result;
use raw_config::RawConfig;
use rswebpack_core::compiler::Compiler;
use rswebpack_core::hooks::BeforeRun;
use rswebpack_core::plugin::driver::PluginDriver;
use rswebpack_core::plugin::{BoxPlugin, PluginExt};
use rswebpack_hook::{plugin, plugin_hook, Hook};
use std::sync::Arc;

#[macro_use]
extern crate napi_derive;

mod plugins;
mod raw_config;

#[napi]
pub struct RsWebpack {
  compiler: Box<Compiler>,
  js_plugin: JsHooksAdapterPlugin,
}

#[napi]
pub struct JsCompiler(pub(crate) &'static mut Compiler);

#[napi]
impl RsWebpack {
  #[napi(constructor)]
  pub fn new(raw_config: RawConfig, register_js_taps: RegisterJsTaps) -> Result<Self> {
    let config = raw_config.try_into().expect("Config transform error");
    let mut plugins = vec![];
    let js_plugin = JsHooksAdapterPlugin::from_js_hooks(register_js_taps)?;
    plugins.push(js_plugin.clone().boxed());
    let mut compiler = Compiler::new(config, plugins);

    // let plugin_driver = Arc::<PluginDriver>::get_mut(&mut compiler.plugin_driver).unwrap();
    // plugin_driver
    //   .compiler_hooks
    //   .before_run
    //   .intercept(RegisterBeforeRunTaps::new(
    //     register_js_taps.register_before_run_taps.clone(),
    //     NonSkippableRegisters::default(),
    //   ));
    Ok(Self {
      compiler: Box::new(compiler),
      js_plugin: js_plugin,
    })
  }

  #[napi]
  pub async unsafe fn run(&mut self) {
    self.compiler.as_mut().run().await;
  }

  #[napi]
  pub fn set_non_skippable_registers(&self, kinds: Vec<RegisterJsTapKind>) {
    self.js_plugin.set_non_skippable_registers(kinds)
  }
}

