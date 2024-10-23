#![allow(clippy::print_stdout)]
use itertools::Itertools;
use oxc_allocator::{Allocator, Vec};
use oxc_ast::ast::*;
use oxc_codegen::{CodeGenerator, CodegenOptions};
use oxc_parser::Parser;
use oxc_semantic::SemanticBuilder;
use oxc_span::SourceType;
use oxc_traverse::{traverse_mut, Ancestor, Traverse, TraverseCtx};
use std::ops::DerefMut;
use std::{env, path::Path, sync::Arc};

struct MyTransform;

impl<'a> Traverse<'a> for MyTransform {
    fn enter_import_declaration(
        &mut self,
        node: &mut ImportDeclaration<'a>,
        ctx: &mut TraverseCtx<'a>,
    ) {
        println!("import declaration {:?}", node)
    }

    // fn enter_variable_declaration(
    //     &mut self,
    //     node: &mut VariableDeclaration<'a>,
    //     ctx: &mut TraverseCtx<'a>,
    // ) {
    //     // node.declarations[0].init;
    //     match &mut node.declarations[0].init {
    //         Some(expression) => match expression {
    //             Expression::CallExpression(call_expression) => {
    //                 println!(
    //                     "require {:?} {:?}",
    //                     call_expression.callee_name(),
    //                     call_expression.arguments[0]
    //                 );
    //                 let e = call_expression.arguments[0].as_expression_mut();
    //             }
    //             _ => {}
    //         },
    //         None => todo!(),
    //     };
    //     // println!("variable declaration {:?}", node.declarations[0].init)
    // }

    fn enter_call_expression(&mut self, node: &mut CallExpression<'a>, ctx: &mut TraverseCtx<'a>) {
        // let vec = Allocator::default();
        // vec.push(Argument::StringLiteral(Box::new(Atom::from("asdg"))));
        // node.arguments = Vec::new::<'a, Argument<'a>>();
        // vec![];
        // println!("{:?}", node.arguments);
        // let exp = node.arguments[0].as_expression_mut();
        let argument = &mut node.arguments.deref_mut()[0];

        match argument {
            Argument::StringLiteral(string_literal) => string_literal.value = Atom::from("asdg"),
            _ => {}
        }
    }

    fn enter_numeric_literal(&mut self, node: &mut NumericLiteral<'a>, ctx: &mut TraverseCtx<'a>) {
        // Read parent
        if let Ancestor::BinaryExpressionRight(bin_expr_ref) = ctx.parent() {
            // This is legal
            if let Expression::Identifier(id) = bin_expr_ref.left() {
                println!("left side is ID: {}", &id.name);
            }

            // This would be a compile failure, because the right side is where we came from
            // dbg!(bin_expr_ref.right());
        }

        // Read grandparent
        if let Ancestor::ExpressionStatementExpression(stmt_ref) = ctx.ancestor(1) {
            // This is legal
            println!("expression stmt's span: {:?}", stmt_ref.span());

            // This would be a compile failure, because the expression is where we came from
            // dbg!(stmt_ref.expression());
        }
    }
}

fn main() -> std::io::Result<()> {
    let name = env::args().nth(1).unwrap_or_else(|| "test.js".to_string());
    let path = Path::new(&name);
    let source_text = Arc::new(std::fs::read_to_string(path)?);
    let source_type = SourceType::from_path(path).unwrap();
    // Memory arena where Semantic and Parser allocate objects
    let allocator = Allocator::default();

    // Parse the source text into an AST
    let parser_ret = Parser::new(&allocator, &source_text, source_type).parse();
    if !parser_ret.errors.is_empty() {
        let error_message: String = parser_ret
            .errors
            .into_iter()
            .map(|error| format!("{:?}", error.with_source_code(Arc::clone(&source_text))))
            .join("\n");
        println!("Parsing failed:\n\n{error_message}",);
        return Ok(());
    }

    let mut program = parser_ret.program;

    // println!("{:?}", program);

    let semantic = SemanticBuilder::new(&source_text)
        .build_module_record(path, &program)
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

    let t = &mut MyTransform;
    let (symbols, scopes) = semantic.semantic.into_symbol_table_and_scope_tree();
    traverse_mut(t, &allocator, &mut program, symbols, scopes);

    let new_code = CodeGenerator::new()
        .with_options(CodegenOptions {
            ..CodegenOptions::default()
        })
        .build(&program)
        .code;

    println!("{}", new_code);

    Ok(())
}
