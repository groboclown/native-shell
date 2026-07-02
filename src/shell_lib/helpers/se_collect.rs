//SPDX:MIT

use crate::shell_lib::structure::{ExitCode, ScriptExit};

/// Gathers ScriptExit objects together to join them into a single value.
/// Unlike using a Vec to store the collected ScriptExit then calling
/// ScriptExit::join, this saves some memory by not keeping the old ScriptExit objects
/// around.
#[derive(Clone)]
pub struct ScriptExitCollector {
    first: bool,
    msg: String,
    err_count: ExitCode,
    pub total: usize,
}

impl ScriptExitCollector {
    /// Create an empty collector.
    pub fn new() -> Self {
        Self {
            first: true,
            msg: String::new(),
            err_count: 0,
            total: 0,
        }
    }

    pub fn option(&mut self, exit: &Option<ScriptExit>) {
        if let Some(exit) = exit {
            self.add(exit)
        }
    }

    pub fn add_result<T>(&mut self, res: Result<T, ScriptExit>) {
        if let Err(exit) = res {
            self.add(&exit)
        }
    }

    pub fn add_se_result(&mut self, res: Result<ScriptExit, ScriptExit>) {
        self.add(match &res {
            Ok(e) => e,
            Err(e) => e,
        });
    }

    pub fn add(&mut self, exit: &ScriptExit) {
        self.total += 1;
        if let Some(message) = &exit.message {
            if self.first {
                self.first = false;
            } else {
                self.msg.push('\n');
            }
            self.msg.push_str(message.as_str());
        }
        if exit.code != 0 {
            self.err_count += 1;
        }
    }

    pub fn close(self) -> ScriptExit {
        if self.first {
            // No messages added.
            ScriptExit {
                code: self.err_count,
                message: None,
            }
        } else {
            ScriptExit {
                code: self.err_count,
                message: Some(self.msg),
            }
        }
    }

    pub fn close_ok(self) -> Result<(), ScriptExit> {
        let res = self.close();
        if res.code == 0 { Ok(()) } else { Err(res) }
    }
}
