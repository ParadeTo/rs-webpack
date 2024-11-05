use crate::compiler::Compiler;
use crate::config::Config;
use rswebpack_macros::{define_hook, plugin, plugin_hook};

define_hook!(Test: SyncSeries(compiler: &mut Compiler));

#[derive(Default, Debug)]
pub struct CompilerHooks {
    pub test: TestHook,
}
