use super::model;
use core::fmt;
use std::collections::HashMap;

/// A validation error that contains a message and the source of the error.
#[derive(Debug, Clone)]
pub struct ValidationError {
    pub message: String,
    pub is_error: bool,
    pub source: model::Source,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?}@{},{}: {}",
            self.source.file, self.source.line, self.source.column, self.message
        )
    }
}

impl ValidationError {
    pub fn new_warning(message: &str, source: &model::Source) -> Self {
        ValidationError {
            message: message.to_string(),
            is_error: false,
            source: source.clone(),
        }
    }
    pub fn new_error(message: &str, source: &model::Source) -> Self {
        ValidationError {
            message: message.to_string(),
            is_error: true,
            source: source.clone(),
        }
    }
}

pub fn validate(lls: &model::NativeShellLowLevelScriptSchema) -> Vec<ValidationError> {
    let mut errors = Vec::new();

    join_validation_vec(&mut errors, validate_jobs(&lls.meta.source, &lls.jobs));
    join_validation_vec(
        &mut errors,
        validate_has_main(&lls.meta.source, &lls.threads),
    );

    // TODO add more validations.

    errors
}

pub fn validate_jobs(
    root: &model::Source,
    jobs: &HashMap<model::NativeShellLowLevelScriptSchemaJobsKey, model::Job>,
) -> Vec<Option<ValidationError>> {
    let mut errors = Vec::new();

    for job in jobs.values() {
        // Validate the jobs's kind is registered

        // TODO more validations
    }

    errors
}

pub fn validate_has_main(
    root: &model::Source,
    threads: &HashMap<model::NativeShellLowLevelScriptSchemaThreadsKey, model::Thread>,
) -> Vec<Option<ValidationError>> {
    for thread in threads.values() {
        if thread.main.is_some() {
            return vec![None]; // Main node found, no error
        }
    }
    vec![Some(ValidationError::new_error(
        "LLS must have at least one thread marked as 'main'",
        root,
    ))]
}

fn ensure_is_id(value: &String, source: &model::Source) -> Option<ValidationError> {
    if value.is_empty() {
        return Some(ValidationError::new_error("ID cannot be empty", source));
    }
    if !value.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Some(ValidationError::new_error(
            "ID must be alphanumeric or underscore",
            source,
        ));
    }
    None
}

fn join_validation(mut src: Vec<ValidationError>, val: Option<ValidationError>) {
    match val {
        Some(v) => src.push(v.clone()),
        None => {}
    }
}

fn join_validation_slice(mut src: Vec<ValidationError>, vals: &[Option<ValidationError>]) {
    for v in vals {
        match v {
            Some(x) => src.push(x.clone()),
            None => {}
        }
    }
}

fn join_validation_vec(src: &mut Vec<ValidationError>, vals: Vec<Option<ValidationError>>) {
    for v in vals {
        match v {
            Some(x) => src.push(x.clone()),
            None => {}
        }
    }
}

fn path_with(base: &Vec<String>, path: &str) -> Vec<String> {
    let mut new_path = base.clone();
    new_path.push(path.to_string());
    new_path
}
