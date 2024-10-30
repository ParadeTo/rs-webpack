use std::collections::HashMap;

use sailfish::TemplateSimple;

#[derive(TemplateSimple)]
#[template(path = "output.stpl")]

pub struct OutputTpl<'a> {
    pub entry_id: &'a str,
    pub modules: &'a HashMap<String, String>,
}
