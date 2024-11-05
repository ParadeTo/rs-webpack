pub mod driver;

use crate::config::Config;
use crate::hooks::CompilerHooks;
use rswebpack_error::Result;

#[async_trait::async_trait]
pub trait Plugin: std::fmt::Debug {
    fn name(&self) -> &'static str {
        "unknown"
    }

    fn apply(&self, _ctx: PluginContext<&mut ApplyContext>) -> Result<()> {
        Ok(())
    }
}

pub type BoxPlugin = Box<dyn Plugin>;

// pub trait PluginExt {
//   fn boxed(self) -> BoxPlugin;
// }
//
// impl<T: Plugin + 'static> PluginExt for T {
//   fn boxed(self) -> BoxPlugin {
//     Box::new(self)
//   }
// }

#[derive(Debug, Default)]
pub struct PluginContext<T = ()> {
    pub context: T,
}

impl PluginContext {
    pub fn new() -> Self {
        Self::with_context(())
    }
}

impl<T> PluginContext<T> {
    pub fn with_context(context: T) -> Self {
        Self { context }
    }

    pub fn into_context(self) -> T {
        self.context
    }
}

#[derive(Debug)]
pub struct ApplyContext<'c> {
    pub compiler_hooks: &'c mut CompilerHooks,
}
