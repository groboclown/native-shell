//! Manually constructed code to show how the builder might turn the AST into a shell program.

use std::collections::HashMap;
use std::sync::{mpsc, Arc};

use crate::shell_lib::compile::job;
use crate::shell_lib::helpers;
use crate::shell_lib::helpers::state_guard::StateGuard;
use crate::shell_lib::modules::{cat, file_sink, shell};
use crate::shell_lib::internal::scheduler;

pub fn main(argv: Vec<String>, environ: HashMap<String, String>) -> i32 {
    // Run the main function and handle any errors.
    env_logger::init();
    let res = run_main(argv, environ);
    if let Err(e) = &res {
        eprintln!("Error: {}", e);
    }
    std::process::exit(res.unwrap_or(1));
}

fn run_main(argv: Vec<String>, environ: HashMap<String, String>) -> Result<i32, String> {
    todo!("Implement the main function logic here");
}
