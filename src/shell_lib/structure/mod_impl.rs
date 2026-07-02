//SPDX:MIT

//! Code bits that the modules referenced in `meta` need to use.

/// The trait that all Command module structures must implement.
///
/// `Params` must match the module's `instance_struct.name` value.
pub trait CommandHandler {
    /// Called when the executable first loads.
    /// After this returns, the first thread will run.
    fn start(
        &self,
        context: Box<dyn super::job::JobRunnerContext>,
    ) -> Result<super::job::ScriptExit, super::job::ScriptExit>;

    /// Called when the main thread completes.
    fn on_exit(
        &self,
        context: Box<dyn super::job::JobRunnerContext>,
        script_exit: super::job::ScriptExit,
    ) -> Result<(), String>;

    /// Called after all the jobs have stopped or encountered a timeout waiting to stop.
    fn on_shutdown(&self) -> Result<(), String>;
}

/// The trait that all Command module structures must implement.
///
/// This contains the parameterized aspects of the command.
pub trait CommandSetup<Params, Struct> {
    /// Called to create the instance.
    fn new(
        source: super::source::Source,
        ctx: &mut dyn super::mod_ctx::InitCtx,
        params: Params,
    ) -> Self;

    fn state(&self) -> &Struct;
}

pub trait CommandImpl<Params, Struct>: CommandHandler + CommandSetup<Params, Struct> {}
