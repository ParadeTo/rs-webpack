pub struct Compilation {
    pub id: u32,
    pub render_hook: RenderHook,
}

pub struct Source {
    pub content: String,
}

pub trait Render {
    fn run(&self, compilation: &Compilation, source: &mut Source) -> Result<(), ()>;
    fn stage(&self) -> i32 {
        0
    }
}
pub struct RenderHook {
    taps: Vec<Box<dyn Render + Send + Sync>>,
    interceptors: Vec<Box<dyn rspack_hook::Interceptor<Self> + Send + Sync>>,
}
impl rspack_hook::Hook for RenderHook {
    type Tap = Box<dyn Render + Send + Sync>;
    fn used_stages(&self) -> rspack_hook::__macro_helper::FxHashSet<i32> {
        rspack_hook::__macro_helper::FxHashSet::from_iter(self.taps.iter().map(|h| h.stage()))
    }
    fn intercept(
        &mut self,
        interceptor: impl rspack_hook::Interceptor<Self> + Send + Sync + 'static,
    ) {
        self.interceptors.push(Box::new(interceptor));
    }
}
impl std::fmt::Debug for RenderHook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RenderHook")
    }
}
impl Default for RenderHook {
    fn default() -> Self {
        Self {
            taps: Default::default(),
            interceptors: Default::default(),
        }
    }
}
impl RenderHook {
    pub fn call(&self, compilation: &Compilation, source: &mut Source) -> Result<(), ()> {
        let mut additional_taps = std::vec::Vec::new();
        for interceptor in self.interceptors.iter() {
            additional_taps.extend(interceptor.call_blocking(self)?);
        }
        let mut all_taps = std::vec::Vec::new();
        all_taps.extend(&self.taps);
        all_taps.extend(&additional_taps);
        all_taps.sort_by_key(|hook| hook.stage());
        for tap in all_taps {
            tap.run(compilation, source)?;
        }
        Ok(())
    }
    pub fn tap(&mut self, tap: impl Render + Send + Sync + 'static) {
        self.taps.push(Box::new(tap));
    }
}

#[derive(Default)]
pub struct MyRenderPlugin {
    inner: ::std::sync::Arc<MyRenderPluginInner>,
}
impl MyRenderPlugin {
    fn new_inner() -> Self {
        Self {
            inner: ::std::sync::Arc::new(MyRenderPluginInner),
        }
    }
    fn from_inner(inner: &::std::sync::Arc<MyRenderPluginInner>) -> Self {
        Self {
            inner: ::std::sync::Arc::clone(inner),
        }
    }
    fn inner(&self) -> &::std::sync::Arc<MyRenderPluginInner> {
        &self.inner
    }
}
impl ::std::ops::Deref for MyRenderPlugin {
    type Target = MyRenderPluginInner;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
#[doc(hidden)]
#[derive(Default)]
pub struct MyRenderPluginInner;

#[allow(non_camel_case_types)]
pub struct render {
    inner: ::std::sync::Arc<MyRenderPluginInner>,
}
impl render {
    pub fn new(plugin: &MyRenderPlugin) -> Self {
        render {
            inner: ::std::sync::Arc::clone(plugin.inner()),
        }
    }
}
impl MyRenderPlugin {
    #[allow(clippy::ptr_arg)]
    fn render(&self, compilation: &Compilation, source: &mut Source) -> Result<(), ()> {
        source.content += "plugin.render";
        source.content += &compilation.id.to_string();
        Ok(())
    }
}
impl ::std::ops::Deref for render {
    type Target = MyRenderPluginInner;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Render for render {
    fn run(&self, compilation: &Compilation, source: &mut Source) -> Result<(), ()> {
        MyRenderPlugin::render(
            &MyRenderPlugin::from_inner(&self.inner),
            compilation,
            source,
        )
    }
}

// #[tokio::main]
// async fn main() -> Result<(), ()> {
//     let mut compilation = Compilation {
//         id: 0,
//         render_hook: RenderHook::default(),
//     };
//     let mut source = Source {
//         content: String::new(),
//     };
//     let plugin = MyRenderPlugin::default();
//     compilation.render_hook.tap(render::new(&plugin));
//     let result = compilation.render_hook.call(&compilation, &mut source);
//     println!("{}", source.content);
//     Ok(())
// }
