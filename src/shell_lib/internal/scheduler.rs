//! The code that manages jobs within the shell.
//! 
//! The scheduler requires the code generation to create a set of jobs and job sequences that may not
//! directly relate to the nodes, modules, or the actions.  For example, an action could construct the streams for other
//! jobs, then trigger them to start through the job scheduler.  The builder must ensure that stateful dependencies like
//! this correctly manage the invocation, so that the stateful sub-jobs only start through the parent.

pub mod run_state;
