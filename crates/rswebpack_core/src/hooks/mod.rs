use crate::compiler::Compiler;
use crate::config::Config;
use rswebpack_macros::{define_hook, plugin, plugin_hook};

define_hook!(BeforeRun: AsyncSeries(compiler: &mut Compiler));
define_hook!(BeforeRunSync: SyncSeries(compiler: &mut Compiler));

#[derive(Default, Debug)]
pub struct CompilerHooks {
    pub before_run: BeforeRunHook,
    pub before_run_sync: BeforeRunSyncHook,
}
