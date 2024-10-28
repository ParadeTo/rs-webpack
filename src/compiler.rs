use itertools::Itertools;
use oxc_allocator::Allocator;
use oxc_codegen::{CodeGenerator, CodegenOptions};
use oxc_parser::Parser;
use oxc_semantic::SemanticBuilder;
use oxc_span::SourceType;
use oxc_traverse::traverse_mut;
use pathdiff::diff_paths;
use sailfish::TemplateSimple;
use std::{
    cell::RefCell,
    collections::HashMap,
    fs::{create_dir_all, read_to_string, File},
    io::Write,
    path::{Path, PathBuf},
    rc::Rc,
    sync::Arc,
    vec,
};

use crate::{config::Config, template::OutputTpl, transform::RsWebpackTransform};

pub struct Compiler {
    config: Config,
    entry_id: String,
    root: String,
    modules: HashMap<String, String>,
    assets: HashMap<String, String>,
}

impl Compiler {
    pub fn new(config: Config) -> Compiler {
        Compiler {
            root: config.root.clone(),
            entry_id: "".to_string(),
            config,
            modules: HashMap::new(),
            assets: HashMap::new(),
        }
    }

    fn parse(
        &self,
        module_path: PathBuf,
        parent_path: &Path,
    ) -> (String, Rc<RefCell<Vec<String>>>) {
        let source_text = Arc::new(read_to_string(&module_path).unwrap());
        let source_type = SourceType::from_path(&module_path).unwrap();
        // Memory arena where Semantic and Parser allocate objects
        let allocator = Allocator::default();

        // 1 Parse the source text into an AST
        let parser_ret = Parser::new(&allocator, &source_text, source_type).parse();
        if !parser_ret.errors.is_empty() {
            let error_message: String = parser_ret
                .errors
                .into_iter()
                .map(|error| format!("{:?}", error.with_source_code(Arc::clone(&source_text))))
                .join("\n");
            panic!("Parsing failed:\n\n{error_message}");
        }

        let mut program = parser_ret.program;

        // 2 Semantic Analyze
        let semantic = SemanticBuilder::new(&source_text)
            .build_module_record(&module_path, &program)
            // Enable additional syntax checks not performed by the parser
            .with_check_syntax_error(true)
            .build(&program);

        if !semantic.errors.is_empty() {
            let error_message: String = semantic
                .errors
                .into_iter()
                .map(|error| format!("{:?}", error.with_source_code(Arc::clone(&source_text))))
                .join("\n");
            println!("Semantic analysis failed:\n\n{error_message}",);
        }
        let (symbols, scopes) = semantic.semantic.into_symbol_table_and_scope_tree();

        // 3 Transform
        let dependencies = Rc::new(RefCell::new(vec![]));
        let rs_webpack_transform = &mut RsWebpackTransform {
            parent_path: parent_path.to_str().unwrap().to_string(),
            dependencies: dependencies.clone(),
        };
        traverse_mut(
            rs_webpack_transform,
            &allocator,
            &mut program,
            symbols,
            scopes,
        );

        // 4 Generate Code
        let new_code = CodeGenerator::new()
            .with_options(CodegenOptions {
                ..CodegenOptions::default()
            })
            .build(&program)
            .code;

        println!("{}", new_code);
        (new_code, dependencies)
    }

    fn build_module(&mut self, module_path: PathBuf, is_entry: bool) {
        let diff_result = diff_paths(&module_path, &self.root).unwrap();
        let module_id = format!("./{}", diff_result.to_str().unwrap());
        let parent_path = Path::new(&module_id).parent().unwrap();

        println!("{:?}", &module_id);

        if is_entry {
            self.entry_id = module_id.clone()
        }

        let (source_code, dependencies) = self.parse(module_path, parent_path);
        self.modules.insert(module_id, source_code);

        for dep in dependencies.borrow().iter() {
            let module_path = Path::new(&self.root).join(dep);
            self.build_module(module_path, false);
        }
    }

    fn emit_file(&mut self) {
        let main = Path::new(&self.config.output.path).join(&self.config.output.filename);
        let ctx = OutputTpl {
            entry_id: self.entry_id.as_str(),
            modules: &self.modules,
        };
        let code = ctx.render_once().unwrap();

        let parent_dir = Path::new(&main).parent().expect("Invalid file path");
        create_dir_all(parent_dir).expect("create dir error");
        let mut file = File::create(&main).expect("create output error");
        file.write_all(code.as_bytes()).expect("write output error");
    }

    pub fn run(&mut self) {
        let resolved_entry = Path::new(&self.root).join(&self.config.entry);
        self.build_module(resolved_entry, true);
        self.emit_file();
    }
}
