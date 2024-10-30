use oxc_ast::ast::*;
use oxc_traverse::{Traverse, TraverseCtx};
use std::{cell::RefCell, ops::DerefMut, path::Path, rc::Rc};

pub struct RsWebpackTransform {
    pub parent_path: String,
    pub dependencies: Rc<RefCell<Vec<String>>>,
}

impl<'a> Traverse<'a> for RsWebpackTransform {
    fn enter_call_expression(&mut self, node: &mut CallExpression<'a>, ctx: &mut TraverseCtx<'a>) {
        if node.is_require_call() {
            match &mut node.callee {
                Expression::Identifier(identifier_reference) => {
                    identifier_reference.name = Atom::from("__webpack_require__")
                }
                _ => {}
            }

            let argument = &mut node.arguments.deref_mut()[0];

            match argument {
                Argument::StringLiteral(string_literal) => {
                    let module_name = string_literal.value.as_str().to_owned();
                    let module_name_with_ext = format!(
                        "{}{}",
                        module_name,
                        match Path::new(&module_name)
                            .extension()
                            .and_then(|ext| ext.to_str())
                        {
                            Some(_) => "",
                            None => ".js",
                        }
                    );
                    let resolved_module_name = if self.parent_path == "." {
                        Path::new(&module_name_with_ext)
                    } else {
                        &Path::new(&self.parent_path).join(&module_name_with_ext)
                    };
                    println!(
                        "resolved_module_name {:?} {:?} {:?} {:?}",
                        self.parent_path, module_name, module_name_with_ext, resolved_module_name
                    );
                    self.dependencies
                        .borrow_mut()
                        .push(resolved_module_name.to_str().unwrap().to_string());

                    string_literal.value = ctx.ast.atom(resolved_module_name.to_str().unwrap());
                }
                _ => {}
            }
        }
    }
}
