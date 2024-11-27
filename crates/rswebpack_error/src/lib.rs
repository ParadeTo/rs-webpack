pub use miette;
use miette::Diagnostic;
use thiserror::Error;
pub type Result<T, E = miette::Error> = std::result::Result<T, E>;

pub type Error = miette::Error;

#[derive(Debug, Error, Diagnostic)]
#[diagnostic()]
#[error("{reason}\n{backtrace}")]
pub struct NodeError {
    pub reason: String,
    pub stack: Option<String>,
    pub backtrace: String,
    pub hide_stack: Option<bool>,
}
