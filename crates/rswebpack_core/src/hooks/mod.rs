use rswebpack_macros::{define_hook, plugin, plugin_hook};
define_hook!(Test: SyncSeries(compiler: &mut Compiler));

struct Hooks {
  test_hook: TestHook
}