pub struct Output {
    pub path: String,
    pub filename: String,
}

pub struct Config {
    pub root: String,
    pub entry: String,
    pub output: Output,
}

impl Config {
    pub fn new(root: String, entry: String, output: Output) -> Config {
        Config {
            root,
            entry,
            output,
        }
    }
}
