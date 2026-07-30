//SPDX:MIT

//! Low-Level Script Definition.
//! The lowest level description of a script that the builder uses to construct the
//! source code for the final build.  Script languages build on top of this, so it
//! acts like the "assembly" level of the make chain:
//!
//!   1. User provides the user script.
//!   2. Script compiler turns the user script into low-level script.
//!   3. NS Builder turns the low-level script into project code (a Rust project, in this case).
//!   4. The make chain ("cargo build", in this case) turns the project code into the final compiled file.

pub mod convert;
pub mod llsio;
pub mod model;
pub mod validate;

