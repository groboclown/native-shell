use std::sync::Arc;

use crate::shell_lib::structure;
use crate::shell_lib::structure::mod_impl::CommandSetup as _;

use super::super::commands;
use super::j0x0;
use super::j0x1;

/// Collection of all jobs.
/// This allows referencing other jobs at execution time.
pub struct Jobs {
    /// The sub-command in-use.
    /// For the purposes of this example, to see how the generated code will look,
    /// this keeps the enum.
    /// However, this should only exist for scripts with sub-commands.  Otherwise,
    /// this will just be the command object itself without matching.
    pub command: SubCommand,

    pub j0x0: j0x0::Jobj0x0,
    pub j0x1: j0x1::Jobj0x1,
}

#[derive(Clone)]
pub struct Runtime {
    pub jobs: Arc<Jobs>,
}

/// Command state enum.
/// For the purposes of this example, to see how the generated code will look,
/// this keeps the enum.
/// However, this should only exist for scripts with sub-commands.  Otherwise,
/// this will just be the state itself without matching.
pub enum CommandState<'a> {
    Cdefault(&'a commands::c_default::CommanddefaultState),
}

/// Command instance.
pub enum SubCommand {
    Cdefault(commands::c_default::Commanddefault),
}

impl Runtime {
    // Command access.
    pub fn command_state<'a>(&'a self) -> CommandState<'a> {
        match &self.jobs.command {
            SubCommand::Cdefault(c) => CommandState::Cdefault(c.state()),
        }
    }
}

impl structure::mod_impl::CommandHandler for Runtime {
    fn start(
        &self,
        context: Box<dyn structure::job::JobRunnerContext>,
    ) -> Result<structure::ScriptExit, structure::ScriptExit> {
        match &self.jobs.command {
            SubCommand::Cdefault(c) => c.start(context),
        }
    }

    fn on_exit(
        &self,
        context: Box<dyn structure::job::JobRunnerContext>,
        script_exit: structure::ScriptExit,
    ) -> Result<(), String> {
        match &self.jobs.command {
            SubCommand::Cdefault(c) => c.on_exit(context, script_exit),
        }
    }

    fn on_shutdown(&self) -> Result<(), String> {
        match &self.jobs.command {
            SubCommand::Cdefault(c) => c.on_shutdown(),
        }
    }
}
