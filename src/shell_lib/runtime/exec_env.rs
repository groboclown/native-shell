//! Execution environment for a node in the evaluated shell.
//! A copy of the information stored in the AST environment,
//! but with values evaluated at runtime.

use std::collections::HashMap;

pub struct Environment {
    pub cwd: String,
    pub environment_variables: HashMap<String, String>,
    pub umode: i64,
}

impl Default for Environment {
    fn default() -> Self {
        Environment {
            cwd: String::new(),
            environment_variables: HashMap::new(),
            umode: 0,
        }
    }
}

impl Environment {
    pub fn new(cwd: String, environment_variables: HashMap<String, String>, umode: i64) -> Self {
        Environment {
            cwd,
            environment_variables,
            umode,
        }
    }
}
