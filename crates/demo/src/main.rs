use rswebpack_core::compiler::Compiler;
use rswebpack_core::config::{Config, Output};
use rswebpack_core::hooks::Test;
use rswebpack_core::plugin::{ApplyContext, Plugin, PluginContext};
use rswebpack_error::Result;
use rswebpack_macros::{plugin, plugin_hook};

#[plugin]
struct TestHookPlugin;

#[plugin_hook(Test for TestHookPlugin)]
fn test(&self, compiler: &mut Compiler) -> Result<()> {
    println!("Root is {}", compiler.root);
    Ok(())
}

#[derive(Debug)]
struct TestPlugin;

impl Plugin for TestPlugin {
    fn apply(&self, _ctx: PluginContext<&mut ApplyContext>) -> Result<()> {
        _ctx.context
            .compiler_hooks
            .test
            .tap(test::new(&TestHookPlugin::new_inner()));
        Ok(())
    }
}

fn main() {
    let config = Config::new(
        "test".to_string(),
        "test".to_string(),
        Output {
            path: "out".to_string(),
            filename: "bundle".to_string(),
        },
    );

    let compiler = &mut Compiler::new(config, vec![Box::new(TestPlugin {})]);
    compiler.run();
}
