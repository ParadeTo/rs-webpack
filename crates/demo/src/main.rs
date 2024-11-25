use rswebpack_binding::interceptor::{
    NonSkippableRegisters, RegisterBeforeRunTaps, RegisterJsTapKind, RegisterJsTaps,
};
use rswebpack_core::compiler::Compiler;
use rswebpack_core::config::{Config, Output};
use rswebpack_core::hooks::{BeforeRun, BeforeRunHook};
use rswebpack_core::plugin::{ApplyContext, Plugin, PluginContext};
use rswebpack_error::Result;
use rswebpack_hook::{Hook, Interceptor};
use rswebpack_macros::{plugin, plugin_hook};

#[plugin]
struct BeforeRunHookTap;

#[plugin_hook(BeforeRun for BeforeRunHookTap)]
fn before_run(&self, compiler: &mut Compiler) -> Result<()> {
    println!("Root is {}", compiler.root);
    Ok(())
}

#[derive(Debug)]
struct TestPlugin;

impl Plugin for TestPlugin {
    fn apply(&self, _ctx: PluginContext<&mut ApplyContext>) -> Result<()> {
        // _ctx.context
        //     .compiler_hooks
        //     .before_run
        //     .intercept(RegisterBeforeRunTaps::new(RegisterBeforeRunTaps::, NonSkippableRegisters::default()));
        _ctx.context
            .compiler_hooks
            .before_run
            .tap(before_run::new(&BeforeRunHookTap::new_inner()));
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
