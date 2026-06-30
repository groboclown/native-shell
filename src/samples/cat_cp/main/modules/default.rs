//! The parameters constructed by the main declaration of the thread 'default'.

/// The default job's parameters.
/// This derives from the parameters and environment variables.
/// The job, being special, uses the same structure for state and initialization parameters.
#[derive(Clone, Debug)]
pub struct JobdefaultParameters {
    pub source: String, // source: script.ns
    pub target: String, // source: script.ns
}
