//SPDX:MIT

//! Simulates the construction of the 'default' module.

use std::{
    ffi::c_int,
    sync::{Arc, Mutex, atomic::AtomicBool},
    thread, time,
};

use clap::{Parser, error::RichFormatter};

use crate::shell_lib::{helpers, structure};

const SIGNAL_WAIT_TIME: time::Duration = time::Duration::from_millis(1);

/// 'default' state values that come from parameters.
#[derive(Clone, Debug, clap::Parser)]
// These extra command macros come from the script's meta section.
#[command(name = "cat-cp")]
#[command(version = "1.0.0")]
#[command(about = None, long_about = "Copies the contents of a source file to a target file.")]
pub struct CommanddefaultArgs {
    // field: 'source'
    // kind: required (implies String type)
    #[arg(long = "source", short = 's')]
    pub source: String,

    // field: 'target'
    // kind: required
    #[arg(long = "target", short = 't')]
    pub target: String,
}

#[derive(Clone, Debug)]
pub struct CommanddefaultState {
    pub args: CommanddefaultArgs,
    // Would also have 'default' state values for envron, but this doesn't take environ.
}

/// Parameters populated by the builder.
pub struct CommanddefaultParameters {
    pub argv: Vec<String>,
    // can also include:
    //   pub envrion: HashMap<String, String>,
    // but this main doesn't use environ.
}

/// The command itself.
pub struct Commanddefault {
    // It does not change over time, so just the state.
    state: CommanddefaultState,

    // Because parameter parsing happens in 'new', this maintains whether the
    // parsing generated an error.
    parse_err: Option<clap::error::Error<RichFormatter>>,

    abort: structure::EventRef,

    stopped: Arc<AtomicBool>,
}

impl structure::mod_impl::CommandSetup<CommanddefaultParameters, CommanddefaultState>
    for Commanddefault
{
    fn new(
        _source: structure::source::Source,
        ctx: &mut dyn structure::mod_ctx::InitCtx,
        params: CommanddefaultParameters,
    ) -> Self {
        // Parse the arguments.
        let mut parse_err = None;
        let args = match CommanddefaultArgs::try_parse_from(params.argv) {
            Ok(a) => a,
            Err(e) => {
                parse_err = Some(e);
                CommanddefaultArgs {
                    source: "".into(),
                    target: "".into(),
                }
            }
        };

        // Register event listeners.
        ctx.callback_on_name(
            helpers::log::TRACE,
            &structure::EventKind::Message,
            Arc::new(Box::new(TraceCb {})),
        );
        ctx.callback_on_name(
            helpers::log::DEBUG,
            &structure::EventKind::Message,
            Arc::new(Box::new(DebugCb {})),
        );
        ctx.callback_on_name(
            helpers::log::VERBOSE,
            &structure::EventKind::Message,
            Arc::new(Box::new(VerboseCb {})),
        );
        ctx.callback_on_name(
            helpers::log::INFO,
            &structure::EventKind::Message,
            Arc::new(Box::new(VerboseCb {})),
        );
        ctx.callback_on_name(
            helpers::log::NOTICE,
            &structure::EventKind::Message,
            Arc::new(Box::new(NoticeCb {})),
        );
        ctx.callback_on_name(
            helpers::log::WARNING,
            &structure::EventKind::Message,
            Arc::new(Box::new(WarningCb {})),
        );
        ctx.callback_on_name(
            helpers::log::ERROR,
            &structure::EventKind::Message,
            Arc::new(Box::new(ErrorCb {})),
        );

        Self {
            state: CommanddefaultState { args },
            parse_err,
            abort: ctx.get_event("abort", &structure::EventKind::Signal),
            stopped: Arc::new(AtomicBool::new(false)),
        }
    }

    fn state(&self) -> &CommanddefaultState {
        &self.state
    }
}

impl structure::mod_impl::CommandHandler for Commanddefault {
    fn start(
        &self,
        ctx: Box<dyn structure::job::JobRunnerContext>,
    ) -> Result<structure::ScriptExit, structure::ScriptExit> {
        if let Some(e) = &self.parse_err {
            eprint!("{}", e);
            return Ok(1.into());
        }

        let ctx = Arc::new(Mutex::new(ctx));

        // OS signal capture.
        let s_channel: Arc<signal_hook::low_level::channel::Channel<(structure::EventRef, c_int)>> =
            Arc::new(signal_hook::low_level::channel::Channel::new());
        {
            let t_ctx = ctx.clone();
            let t_chan = s_channel.clone();
            let t_stopped = self.stopped.clone();
            std::thread::spawn(move || {
                loop {
                    match t_chan.recv() {
                        Some((e, s)) => {
                            let _ = t_ctx.lock().map_err(|e| e.into()).and_then(|c| {
                                c.send_event(e, structure::EventPayload::Signal(s.into()))
                            });
                            if t_stopped.load(std::sync::atomic::Ordering::SeqCst) {
                                break;
                            }
                        }
                        None => {
                            // Too long and this can mix signals.
                            thread::sleep(SIGNAL_WAIT_TIME);
                        }
                    };
                }
            });
        }

        // Group signals by their event.
        {
            // Abort handler.
            let t_chan = s_channel.clone();
            let t_event = self.abort;
            let t_stopped = self.stopped.clone();
            let mut signals = signal_hook::iterator::Signals::new(&[
                signal_hook::consts::signal::SIGINT,
                signal_hook::consts::signal::SIGTERM,
                signal_hook::consts::signal::SIGSTOP,
            ])?;
            std::thread::spawn(move || {
                // Unfortunately, this has either 'wait until an event' or
                // 'poll', and no 'wait with timeout'.
                loop {
                    for sig in signals.pending() {
                        t_chan.send((t_event, sig));
                    }
                    if t_stopped.load(std::sync::atomic::Ordering::SeqCst) {
                        break;
                    }
                    thread::sleep(SIGNAL_WAIT_TIME);
                }
            });
        }

        Ok(0.into())
    }

    fn on_exit(
        &self,
        ctx: Box<dyn structure::job::JobRunnerContext>,
        _script_exit: structure::job::ScriptExit,
    ) -> Result<(), String> {
        self.stopped
            .store(true, std::sync::atomic::Ordering::SeqCst);
        ctx.send_event(self.abort, structure::EventPayload::Signal(0))
            .map_err(|e| format!("{}", e))?;
        Ok(())
    }

    fn on_shutdown(&self) -> Result<(), String> {
        Ok(())
    }
}

// --------------------------------------------------------------------
// Event Listeners

struct TraceCb();

impl structure::EventCallback for TraceCb {
    fn on<'a, 'b, 'c>(
        &'a self,
        _context: &'b Box<dyn structure::job::JobRunnerContext>,
        _event_ref: structure::EventRef,
        payload: &'c structure::EventPayload,
    ) -> Result<(), structure::ScriptExit> {
        match payload {
            structure::EventPayload::Message(msg) => log::trace!("{}", msg),
            structure::EventPayload::Signal(_) => (),
        }
        Ok(())
    }
}

struct DebugCb();

impl structure::EventCallback for DebugCb {
    fn on<'a, 'b, 'c>(
        &'a self,
        _context: &'b Box<dyn structure::job::JobRunnerContext>,
        _event_ref: structure::EventRef,
        payload: &'c structure::EventPayload,
    ) -> Result<(), structure::ScriptExit> {
        match payload {
            structure::EventPayload::Message(msg) => log::debug!("{}", msg),
            structure::EventPayload::Signal(_) => (),
        }
        Ok(())
    }
}

struct VerboseCb();

impl structure::EventCallback for VerboseCb {
    fn on<'a, 'b, 'c>(
        &'a self,
        _context: &'b Box<dyn structure::job::JobRunnerContext>,
        _event_ref: structure::EventRef,
        payload: &'c structure::EventPayload,
    ) -> Result<(), structure::ScriptExit> {
        match payload {
            structure::EventPayload::Message(msg) => log::info!("{}", msg),
            structure::EventPayload::Signal(_) => (),
        }
        Ok(())
    }
}

struct InfoCb();

impl structure::EventCallback for InfoCb {
    fn on<'a, 'b, 'c>(
        &'a self,
        _context: &'b Box<dyn structure::job::JobRunnerContext>,
        _event_ref: structure::EventRef,
        payload: &'c structure::EventPayload,
    ) -> Result<(), structure::ScriptExit> {
        match payload {
            structure::EventPayload::Message(msg) => log::info!("{}", msg),
            structure::EventPayload::Signal(_) => (),
        }
        Ok(())
    }
}

struct NoticeCb();

impl structure::EventCallback for NoticeCb {
    fn on<'a, 'b, 'c>(
        &'a self,
        _context: &'b Box<dyn structure::job::JobRunnerContext>,
        _event_ref: structure::EventRef,
        payload: &'c structure::EventPayload,
    ) -> Result<(), structure::ScriptExit> {
        // TODO this needs to capture the message and ensure it doesn't show up
        //      more than once.
        match payload {
            structure::EventPayload::Message(msg) => log::warn!("{}", msg),
            structure::EventPayload::Signal(_) => (),
        }
        Ok(())
    }
}

struct WarningCb();

impl structure::EventCallback for WarningCb {
    fn on<'a, 'b, 'c>(
        &'a self,
        _context: &'b Box<dyn structure::job::JobRunnerContext>,
        _event_ref: structure::EventRef,
        payload: &'c structure::EventPayload,
    ) -> Result<(), structure::ScriptExit> {
        match payload {
            structure::EventPayload::Message(msg) => log::warn!("{}", msg),
            structure::EventPayload::Signal(_) => (),
        }
        Ok(())
    }
}

struct ErrorCb();

impl structure::EventCallback for ErrorCb {
    fn on<'a, 'b, 'c>(
        &'a self,
        _context: &'b Box<dyn structure::job::JobRunnerContext>,
        _event_ref: structure::EventRef,
        payload: &'c structure::EventPayload,
    ) -> Result<(), structure::ScriptExit> {
        match payload {
            structure::EventPayload::Message(msg) => log::error!("{}", msg),
            structure::EventPayload::Signal(_) => (),
        }
        Ok(())
    }
}
