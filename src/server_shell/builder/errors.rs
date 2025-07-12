//! General errors that come from turning the AST into a module source.


pub enum BuilderError {
    /// Error when the AST is invalid.
    InvalidAst(String),
    /// Error when a module is not found.
    ModuleNotFound(String),
    /// Error when a node is not found.
    NodeNotFound(String),
    /// Error when a target stream is not found.
    StreamNotFound(String),
    /// Error when a node has no streams.
    NodeHasNoStreams(String),
}
