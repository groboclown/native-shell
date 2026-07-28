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
                Self::push_err(err, &mut e);
            }
            Err(mut e) => {
                Self::push_err(err, e.get_mut());
            }
        }
    }

    fn push_err(err: BuilderError, v: &mut Vec<BuilderError>) {
        let mut stack = vec![err];
        loop {
            if let Some(err) = stack.pop() {
                match err {
                    BuilderError::Collection(mut c) => {
                        stack.append(&mut c);
                    }
                    _ => {
                        v.push(err);
                    }
                }
            } else {
                break;
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
        match res {
            Ok(v) => Some(v),
            Err(e) => {
                self.add_err(e);
                None
            }
        }
    }

    /// Consumes the error for functions that return Result<(), ()>.
    pub fn consume<T>(&self, res: Result<T, BuilderError>) -> Result<T, ()> {
        match res {
            Ok(v) => Ok(v),
            Err(e) => {
                self.add_err(e);
                Err(())
            }
        }
    }

    pub fn add_errs(&self, mut issues: Vec<BuilderError>) {
        for err in issues.drain(0..issues.len()) {
            self.add_err(err);
        }
    }
}

impl Into<BuilderError> for ScriptIssues {
    fn into(self) -> BuilderError {
        match self.errs.lock() {
            Ok(m) if m.len() == 1 => m.first().unwrap().clone(),
            Ok(m) => BuilderError::Collection(m.clone()),
            Err(e) => {
                let e = (*e.get_ref()).clone();
                if e.len() == 1 {
                    e.first().unwrap().clone()
                } else {
                    BuilderError::Collection(e)
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Relationship {
    StreamSource,
    StreamTarget,
    Definition,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RelatedSource {
    pub relation: Relationship,
    pub source: ErrSource,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ErrorDetails {
    pub message: String,
    pub source: ErrSource,
    pub related: Vec<RelatedSource>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum BuilderError {
    /// Error when the LLS is invalid.
    InvalidLLS(ErrorDetails),
    /// Error when a module is not found.
    ModuleNotRegistered(ErrorDetails),
    /// Error when a macro is not found.
    MacroNotRegistered(ErrorDetails),
    MacroNotUsableForCommand(ErrorDetails),
    MacroNotUsableForJob(ErrorDetails),
    ModuleNotUsableForCommand(ErrorDetails),
    ModuleNotUsableForJob(ErrorDetails),
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
    IOError(String), //std::io::Error
    // Zip Error
    ZipError(String), // zip::result::ZipError
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
    /// Something generated a panic.
    General(String),
}

impl From<std::io::Error> for BuilderError {
    fn from(value: std::io::Error) -> Self {
        BuilderError::IOError(value.to_string())
    }
}

impl From<zip::result::ZipError> for BuilderError {
    fn from(value: zip::result::ZipError) -> Self {
        BuilderError::ZipError(value.to_string())
    }
}

/// Send the error to stderr.
pub fn report_errors(err: &BuilderError) {
    match err {
        BuilderError::InvalidLLS(details) => {
            eprintln!("Invalid LLS: {}", details.message);
            show_source(&details.source);
            show_related(&details);
        }
        BuilderError::LLSBug(error_details) => {
            eprintln!("Bug in the LLS construction: {}", error_details.message);
            show_source(&error_details.source);
            show_related(error_details);
        }
        BuilderError::ModuleNotRegistered(details) => {
            eprintln!("Referenced unknown module: {}", details.message);
            show_source(&details.source);
            show_related(&details);
        }
        BuilderError::MacroNotRegistered(error_details) => {
            eprintln!("Referenced unknown macro: {}", error_details.message);
            show_source(&error_details.source);
            show_related(error_details);
        }
        BuilderError::MacroNotUsableForCommand(error_details) => {
            eprintln!("Macro cannot apply to commands: {}", error_details.message);
            show_source(&error_details.source);
            show_related(error_details);
        }
        BuilderError::MacroNotUsableForJob(error_details) => {
            eprintln!("Macro cannot apply to jobs: {}", error_details.message);
            show_source(&error_details.source);
            show_related(error_details);
        }
        BuilderError::ModuleNotUsableForCommand(error_details) => {
            eprintln!("Module cannot apply to commands: {}", error_details.message);
            show_source(&error_details.source);
            show_related(error_details);
        }
        BuilderError::ModuleNotUsableForJob(error_details) => {
            eprintln!("Module cannot apply to jobs: {}", error_details.message);
            show_source(&error_details.source);
            show_related(error_details);
        }
        BuilderError::NoSuchJob(details) => {
            eprintln!("No such job: {}", details.message);
            show_source(&details.source);
            show_related(&details);
        }
        BuilderError::JobCommandOverlap(error_details) => {
            eprintln!(
                "Job and Command share the same name: {}",
                error_details.message
            );
            show_source(&error_details.source);
            show_related(error_details);
        }
        BuilderError::NoSuchThread(error_details) => {
            eprintln!("Referenced unknown thread: {}", error_details.message);
            show_source(&error_details.source);
            show_related(error_details);
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
        BuilderError::EventKindMismatch(error_details) => {
            eprintln!(
                "Events referenced different kinds: {}",
                error_details.message
            );
            show_source(&error_details.source);
            show_related(error_details);
        }
        BuilderError::IOError(e) => {
            eprintln!("I/O error: {}", e);
        }
        BuilderError::ZipError(e) => {
            eprintln!("Zip file error: {}", e);
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
        BuilderError::General(msg) => {
            eprintln!("{}", msg);
        }
    }
}

fn show_source(source: &ErrSource) {
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
    eprint!("{}", source.file);
    if let Some(line) = source.line {
        eprint!(", line {}", line);
        if let Some(column) = source.column {
            eprint!(", column {}", column);
        }
    }
    eprintln!();
    show_line(source);
}

fn show_line(source: &ErrSource) {
    match &source.text {
        Some(t) => {
            eprintln!("{}", *t);
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
                Relationship::Definition => "Source definition",
            }
        );
        show_source(&related.source);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ErrSource {
    pub file: String,
    pub line: Option<i64>,
    pub column: Option<i64>,
    pub text: Option<String>,
}

impl From<Source> for ErrSource {
    fn from(value: Source) -> Self {
        Self {
            file: (*value.file).clone(),
            line: value.line.clone(),
            column: value.column.clone(),
            text: match value.text {
                Some(v) => Some((*v).clone()),
                None => None,
            },
        }
    }
}

impl From<&Source> for ErrSource {
    fn from(value: &Source) -> Self {
        Self {
            file: (*value.file).clone(),
            line: value.line.clone(),
            column: value.column.clone(),
            text: match &value.text {
                Some(v) => Some((**v).clone()),
                None => None,
            },
        }
    }
}
