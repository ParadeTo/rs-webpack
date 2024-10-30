use std::error::Error;

use rs_webpack_core::config::{Config, Output};

#[napi(object)]
pub struct RawOutput {
  pub path: String,
  pub filename: String,
}

impl TryFrom<RawOutput> for Output {
  type Error = ();

  fn try_from(value: RawOutput) -> Result<Self, Self::Error> {
    Ok(Output {
      path: value.path.into(),
      filename: value.filename.into(),
    })
  }
}

#[napi(object)]
pub struct RawConfig {
  pub root: String,
  pub entry: String,
  pub output: RawOutput,
}

impl TryFrom<RawConfig> for Config {
  type Error = ();

  fn try_from(value: RawConfig) -> Result<Self, Self::Error> {
    Ok(Config {
      root: value.root.into(),
      entry: value.entry.into(),
      output: value.output.try_into()?,
    })
  }
}
