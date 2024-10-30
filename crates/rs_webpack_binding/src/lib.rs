#![deny(clippy::all)]

use napi::Result;
use raw_config::RawConfig;
use rs_webpack_core::compiler::Compiler;
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
    Ok(Self {
      compiler: Box::new(Compiler::new(config)),
    })
  }

  #[napi]
  pub fn run(&mut self) {
    self.compiler.as_mut().run();
  }
}
