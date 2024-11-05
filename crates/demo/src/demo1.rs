use rswebpack_error::Result;
use rswebpack_macros::{define_hook, plugin, plugin_hook};

define_hook!(Render: SyncSeries(compilation: &Compilation, source: &mut Source));

struct Compilation {
    id: u32,
    render_hook: RenderHook,
}

struct Source {
    content: String,
}

#[plugin]
#[derive(Default)]
struct MyRenderPlugin1 {
    name: String,
}

#[plugin_hook(Render for MyRenderPlugin1)]
fn render1(&self, compilation: &Compilation, source: &mut Source) -> Result<()> {
    source.content += &self.name;
    source.content += &compilation.id.to_string();
    Ok(())
}

#[plugin]
#[derive(Default)]
struct MyRenderPlugin2 {
    name: String,
}

#[plugin_hook(Render for MyRenderPlugin2)]
fn render2(&self, compilation: &Compilation, source: &mut Source) -> Result<()> {
    source.content += &self.name;
    source.content += &compilation.id.to_string();
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut compilation = Compilation {
        id: 0,
        render_hook: RenderHook::default(),
    };
    let mut source = Source {
        content: String::new(),
    };
    let plugin1 = MyRenderPlugin1::new_inner("plugin1".to_string());
    let plugin2 = MyRenderPlugin2::new_inner("plugin2".to_string());
    compilation.render_hook.tap(render1::new(&plugin1));
    compilation.render_hook.tap(render2::new(&plugin2));
    compilation.render_hook.call(&compilation, &mut source);
    println!("{}", source.content);
    Ok(())
}
