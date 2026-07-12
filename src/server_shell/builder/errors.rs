//! General errors that come from turning the AST into a module source.

use crate::server_shell::lls::model::Source;

#[derive(Clone, Debug)]
pub enum Relationship {
    StreamSource,
    StreamTarget,
    //SpawnsNode,
    //SpawnedByNode,
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
    InvalidStreamUse(ErrorDetails),
    /// A cycle happened in the streams.
    StreamCycle(ErrorDetails),
    /// I/O error.
    IOError(std::io::Error),
    /// Zip Error
    ZipError(zip::result::ZipError),
    /// Many errors.
    Collection(Vec<BuilderError>),
    /// The node's module does not have a state struct.
    NoStateForModule(ErrorDetails),
    /// The node's module's state field type does not match the expected type.
    FieldTypeMismatch(ErrorDetails),
    /// The node's module does not have a state field with the given name.
    NoSuchField(ErrorDetails),
    /// Bug.  Something shoved this in the wrong sequence.
    MainModuleInSequence(ErrorDetails),
    /// Invalid AST range specifier.
    InvalidRange(ErrorDetails),
    /// A required field is missing in the module's structure.
    RequiredFieldMissing(ErrorDetails),
    /// HTTP error.
    HttpError(String),
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
        BuilderError::InvalidStreamUse(details) => {
            eprintln!("Invalid stream use: {}", details.message);
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
        BuilderError::ZipError(e) => {
            eprintln!("Zip error: {}", e);
        }
        BuilderError::NoStateForModule(error_details) => {
            eprintln!("No state for module: {}", error_details.message);
            show_source(&error_details.source);
            show_related(error_details);
        }
        BuilderError::FieldTypeMismatch(error_details) => {
            eprintln!("Field type mismatch: {}", error_details.message);
            show_source(&error_details.source);
            show_related(error_details);
        }
        BuilderError::NoSuchField(error_details) => {
            eprintln!("No such field: {}", error_details.message);
            show_source(&error_details.source);
            show_related(error_details);
        }
        BuilderError::MainModuleInSequence(error_details) => {
            eprintln!("BUG Main module in sequence: {}", error_details.message);
            show_source(&error_details.source);
            show_related(error_details);
        }
        BuilderError::RequiredFieldMissing(error_details) => {
            eprintln!("Required field missing: {}", error_details.message);
            show_source(&error_details.source);
            show_related(error_details);
        }
        BuilderError::InvalidRange(error_details) => {
            eprintln!("Invalid range specifier: {}", error_details.message);
            show_source(&error_details.source);
            show_related(error_details);
        }
        BuilderError::HttpError(msg) => {
            eprintln!("HTTP error: {}", msg);
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
        if let Some(line) = source.line {
            eprint!("Line {}", line);
            if let Some(column) = source.column {
                eprint!(", column {}", column);
            }
            eprintln!();
        }
        show_line(source);
        return;
    }
    eprint!("{}", *source.file);
    if let Some(line) = source.line {
        eprint!(", line {}", line);
        if let Some(column) = source.column {
            eprint!(", column {}", column);
        }
    }
    eprintln!();
    show_line(source);
}

fn show_line(source: &Source) {
    match &source.text {
        Some(t) => {
            eprintln!("{}", **t);
            if let Some(column) = source.column {
                for _ in 0..(column - 1) {
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
                //Relationship::SpawnsNode => "Spawns node",
                //Relationship::SpawnedByNode => "Spawned by node",
                Relationship::StreamSource => "Stream source",
                Relationship::StreamTarget => "Stream target",
            }
        );
        show_source(&related.source);
    }
}
