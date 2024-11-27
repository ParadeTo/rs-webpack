use crate::config::Config;
use crate::hooks::CompilerHooks;
use crate::plugin::{ApplyContext, Plugin, PluginContext};
use std::sync::Arc;

pub struct PluginDriver {
    pub plugins: Vec<Box<dyn Plugin>>,
    pub compiler_hooks: CompilerHooks,
}

impl PluginDriver {
    pub fn new(plugins: Vec<Box<dyn Plugin>>) -> Arc<Self> {
        let mut compiler_hooks = CompilerHooks::default();
        let mut apply_context = ApplyContext {
            compiler_hooks: &mut compiler_hooks,
        };

        for plugin in &plugins {
            plugin
                .apply(PluginContext::with_context(&mut apply_context))
                .expect("failed to apply plugin context");
        }

        Arc::new(Self {
            plugins,
            compiler_hooks,
        })
    }
}
