//SPDX:MIT

//! TODO MAJOR REFACTOR
//!
//! This should perform taking the LLS and generate:
//!
//!  * Map of thread names -> parsed threads.
//!  * Map of job names -> stream structures.
//!    * Stream jobs have very special semantics and build properties.
//!  * Map of job names -> job structure.  The job structure contains:
//!    * Simplified version of runtime parameters to allow the later tooling to
//!      turn this into code.  This also allows for gathering references to other
//!      jobs.
//!    * Simplified version of the compile-time parameters.
//!    * Module or macro reference.
//!    * For macros, it will also generate the corresponding ModuleMeta for it + the
//!      macro files.
//!  * List of all module and macros used in the jobs.
//!  * List of all job names that are unused (not referenced by other jobs or threads).
//!  * List of all referenced job and thread names and job state references that do
//!    not exist.
