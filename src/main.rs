use std::{env, path::Path};

use config::{Config, Output};
use rs_compiler::Compiler;

mod config;
mod demo;
mod rs_compiler;
mod template;
mod transform;

fn main() -> std::io::Result<()> {
    let cwd = env::current_dir().unwrap();
    let config = Config::new(
        cwd.join("js_code").to_str().unwrap().to_owned(),
        "index.js".to_owned(),
        Output {
            path: "./out".to_owned(),
            filename: "bundle.js".to_owned(),
        },
    );
    let compiler = &mut Compiler::new(config);
    compiler.run();
    Ok(())
}
