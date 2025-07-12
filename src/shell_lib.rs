//! Library used by the generated code to help with shell-like operations.
//! 
//! It contains runtime data structures passed to the modules from the
//! generated code, the module implementations, and helper functions for use
//! by the modules and generated code.

pub mod compile;
pub mod helpers;
pub mod modules;
pub mod runtime;

mod internal;
