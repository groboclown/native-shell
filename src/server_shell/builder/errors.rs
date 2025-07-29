//! General errors that come from turning the AST into a module source.

use crate::server_shell::ast::model::Source;

pub struct ErrorDetails {
    pub message: String,
    pub source: Source,
    pub related: Vec<Source>,
}


pub enum BuilderError {
    /// Error when the AST is invalid.
    InvalidAst(ErrorDetails),
    /// Error when a module is not found.
    ModuleNotFound(ErrorDetails),
    /// Error when a node is not found.
    NoSuchNode(ErrorDetails),
    /// Error when a target stream is not found.
    StreamNotFound(ErrorDetails),
    /// A stream is referenced by multiple nodes.
    MultipleStreamUse(ErrorDetails),
    /// I/O error.
    IOError(std::io::Error),
}
