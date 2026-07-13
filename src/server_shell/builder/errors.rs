//! General errors that come from turning the AST into a module source.

use std::sync::{Arc, Mutex};

use crate::server_shell::lls::model::Source;

#[derive(Clone, Debug)]
pub struct ScriptIssues {
    errs: Arc<Mutex<Vec<BuilderError>>>,
}

impl ScriptIssues {
    pub fn new() -> Self {
        Self {
            errs: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn is_ok(&self) -> bool {
        match self.errs.lock() {
            Ok(e) => (*e).is_empty(),
            Err(e) => (*e.get_ref()).is_empty(),
        }
    }

    pub fn has_issues(&self) -> bool {
        !self.is_ok()
    }

    pub fn add_err(&self, err: BuilderError) {
        match self.errs.lock() {
            Ok(mut e) => {
                e.push(err);
            }
            Err(mut e) => {
                e.get_mut().push(err);
            }
        }
    }

    pub fn get(&self) -> Vec<BuilderError> {
        match self.errs.lock() {
            Ok(e) => (*e).clone(),
            Err(e) => (*e.get_ref()).clone(),
        }
    }

    pub fn add_issues(&self, issues: &Self) {
        self.add_errs(issues.get());
    }

    pub fn add_result<T>(&self, res: Result<T, BuilderError>) -> Option<T> {
        if let Err(e) = res {
            self.add_err(e);
        }
        res.ok()
    }

    pub fn add_errs(&self, mut issues: Vec<BuilderError>) {
        match self.errs.lock() {
            Ok(mut e) => {
                for err in issues.drain(0..issues.len()) {
                    e.push(err);
                }
            }
            Err(mut e) => {
                let e = e.get_mut();
                for err in issues.drain(0..issues.len()) {
                    e.push(err);
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum Relationship {
    StreamSource,
    StreamTarget,
    Definition,
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

#[derive(Clone, Debug)]
pub enum BuilderError {
    /// Error when the LLS is invalid.
    InvalidLLS(ErrorDetails),
    /// Error when a module is not found.
    ModuleNotRegistered(ErrorDetails),
    /// Error when a macro is not found.
    MacroNotRegistered(ErrorDetails),
    /// Error when a job or command is not found.
    NoSuchJob(ErrorDetails),
    /// Error when a job and command share the same name.
    JobCommandOverlap(ErrorDetails),
    /// Error when a thread is not found.
    NoSuchThread(ErrorDetails),
    /// Error when a target stream is not found.
    StreamNotFound(ErrorDetails),
    /// A stream is referenced by multiple nodes.
    InvalidStreamUse(ErrorDetails),
    /// A cycle happened in the streams.
    StreamCycle(ErrorDetails),
    // I/O error.
    //IOError(std::io::Error),
    // Zip Error
    //ZipError(zip::result::ZipError),
    /// Many errors.
    Collection(Vec<BuilderError>),
    /// The node's module does not have a state struct.
    NoStateForModule(ErrorDetails),
    /// The node's module's state field type does not match the expected type.
    FieldTypeMismatch(ErrorDetails),
    /// The node's module does not have a state field with the given name.
    NoSuchField(ErrorDetails),
    /// A thread tried starting or waiting on a command.
    CommandReferencedInThread(ErrorDetails),
    /// Invalid AST range specifier.
    InvalidRange(ErrorDetails),
    /// A required field is missing in the module's structure.
    RequiredFieldMissing(ErrorDetails),
    /// HTTP error.
    HttpError(String),
    /// A bug happened while processing the LLS.
    LLSBug(ErrorDetails),
    /// Something referenced an event of type A, but something else used it as type B
    EventKindMismatch(ErrorDetails),
}

//impl From<std::io::Error> for BuilderError {
//    fn from(value: std::io::Error) -> Self {
//        BuilderError::IOError(value)
//    }
//}

pub fn report_errors(err: &BuilderError) {
    match err {
        BuilderError::InvalidLLS(details) => {
            eprintln!("Invalid LLS: {}", details.message);
            show_source(&details.source);
            show_related(&details);
        }
        BuilderError::ModuleNotRegistered(details) => {
            eprintln!("Module not registered: {}", details.message);
            show_source(&details.source);
            show_related(&details);
        }
        BuilderError::NoSuchJob(details) => {
            eprintln!("No such job: {}", details.message);
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
        //BuilderError::IOError(e) => {
        //    eprintln!("I/O error: {}", e);
        //}
        //BuilderError::ZipError(e) => {
        //    eprintln!("Zip error: {}", e);
        //}
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
        BuilderError::CommandReferencedInThread(error_details) => {
            eprintln!("Command referenced in thread: {}", error_details.message);
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
                Relationship::StreamSource => "Stream source",
                Relationship::StreamTarget => "Stream target",
            }
        );
        show_source(&related.source);
    }
}
