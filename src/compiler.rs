use std::{env, fs, path::PathBuf};

pub struct Config {
    entry: String,
}

impl Config {
    pub fn new(entry: String) -> Config {
        Config { entry }
    }
}

pub struct Compiler {
    config: Config,
    root: PathBuf,
}

impl Compiler {
    pub fn new(config: Config) -> Compiler {
        Compiler {
            config,
            root: env::current_dir().expect("msg"),
        }
    }

    fn get_source(&self, module_path: PathBuf) {
        let content =
            fs::read_to_string(module_path).expect("Should have been able to read the file");
        println!("{}", content);
    }

    fn build_module(&self, module_path: PathBuf, is_entry: bool) {
        self.get_source(module_path);
    }

    pub fn run(&self) {
        let resolved_entry = self.root.join(&self.config.entry);
        self.build_module(resolved_entry, true);
    }
}
