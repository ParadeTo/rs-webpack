#![deny(clippy::all)]

use napi::Result;
use raw_config::RawConfig;
use rswebpack_core::compiler::Compiler;
use rswebpack_core::plugin::BoxPlugin;

#[macro_use]
extern crate napi_derive;

mod raw_config;

#[napi]
pub struct RsWebpack {
  compiler: Box<Compiler>,
}

#[napi]
impl RsWebpack {
  #[napi(constructor)]
  pub fn new(raw_config: RawConfig) -> Result<Self> {
    let config = raw_config.try_into().expect("Config transform error");
    let plugins = vec![];
    Ok(Self {
      compiler: Box::new(Compiler::new(config, plugins)),
    })
  }

  #[napi]
  pub fn run(&mut self) {
    self.compiler.as_mut().run();
  }
}
