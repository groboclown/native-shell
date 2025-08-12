//! General errors that come from turning the AST into a module source.

use crate::server_shell::ast::model::Source;

#[derive(Clone, Debug)]
pub enum Relationship {
    StreamSource,
    StreamTarget,
    SpawnsNode,
    SpawnedByNode,
}

#[derive(Clone, Debug)]
pub struct RelatedSource {
    pub relation: Relationship,
    pub source: Source,
}

#[derive(Clone, Debug)]
pub struct ErrorDetails {
    pub message: String,
    pub source: Source,
    pub related: Vec<RelatedSource>,
}

#[derive(Debug)]
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
    /// A cycle happened in the streams.
    StreamCycle(ErrorDetails),
    /// I/O error.
    IOError(std::io::Error),
    /// Many errors.
    Collection(Vec<BuilderError>),
    /// The node's module does not have a state struct.
    NoStateForModule(ErrorDetails),
    /// The node's module's state field type does not match the expected type.
    StateFieldTypeMismatch(ErrorDetails),
    /// The node's module does not have a state field with the given name.
    NoSuchStateField(ErrorDetails),
}

impl From<std::io::Error> for BuilderError {
    fn from(value: std::io::Error) -> Self {
        BuilderError::IOError(value)
    }
}

pub fn report_errors(err: &BuilderError) {
    match err {
        BuilderError::InvalidAst(details) => {
            eprintln!("Invalid AST: {}", details.message);
            show_source(&details.source);
            show_related(&details);
        }
        BuilderError::ModuleNotFound(details) => {
            eprintln!("Module not found: {}", details.message);
            show_source(&details.source);
            show_related(&details);
        }
        BuilderError::NoSuchNode(details) => {
            eprintln!("No such node: {}", details.message);
            show_source(&details.source);
            show_related(&details);
        }
        BuilderError::StreamNotFound(details) => {
            eprintln!("Stream not found: {}", details.message);
            show_source(&details.source);
            show_related(&details);
        }
        BuilderError::MultipleStreamUse(details) => {
            eprintln!("Multiple stream use: {}", details.message);
            show_source(&details.source);
            show_related(&details);
        }
        BuilderError::StreamCycle(details) => {
            eprintln!("Streams point back on each other: {}", details.message);
            show_source(&details.source);
            show_related(&details);
        }
        BuilderError::IOError(e) => {
            eprintln!("I/O error: {}", e);
        }
        BuilderError::NoStateForModule(error_details) => {
            eprintln!("No state for module: {}", error_details.message);
            show_source(&error_details.source);
            show_related(error_details);
        },
        BuilderError::StateFieldTypeMismatch(error_details) => {
            eprintln!("State field type mismatch: {}", error_details.message);
            show_source(&error_details.source);
            show_related(error_details);
        }
        BuilderError::NoSuchStateField(error_details) => {
            eprintln!("No such state field: {}", error_details.message);
            show_source(&error_details.source);
            show_related(error_details);
        }
        BuilderError::Collection(errs) => {
            for err in errs {
                report_errors(err);
            }
        }
    }
}

fn show_source(source: &Source) {
    if source.file.is_empty() {
        if source.line > 0 {
            eprint!("Line {}", source.line);
            if source.column > 0 {
                eprint!(", column {}", source.column);
            }
            eprintln!();
        }
        show_line(source);
        return;
    }
    eprint!("{}", source.file);
    if source.line > 0 {
        eprint!(", line {}", source.line);
        if source.column > 0 {
            eprint!(", column {}", source.column);
        }
    }
    eprintln!();
    show_line(source);
}

fn show_line(source: &Source) {
    match &source.text {
        Some(t) => {
            eprintln!("{}", t);
            if source.column > 0 {
                for _ in 0..(source.column - 1) {
                    eprint!("-");
                }
                eprintln!("^");
            }
        }
        None => {}
    }
}

fn show_related(details: &ErrorDetails) {
    for related in &details.related {
        eprintln!(
            "{}:",
            match related.relation {
                Relationship::SpawnsNode => "Spawns node",
                Relationship::SpawnedByNode => "Spawned by node",
                Relationship::StreamSource => "Stream source",
                Relationship::StreamTarget => "Stream target",
            }
        );
        show_source(&related.source);
    }
}
