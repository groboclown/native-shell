//! The generated module that comes from job j0x0, which
//! constructs the streams for thread jt1.
//! The state for this contains the streams that it created after running.
//! This has a special buildup mechanism to allow the downstream jobs to
//! take the stream, which differs from the standard job types.

use std::sync::{Arc, Mutex};

use crate::shell_lib::{helpers, stream, structure};

#[derive(Clone)]
pub struct Jobj0x0State {
    inner: Arc<Mutex<Jobj0x0StateInner>>,
}

impl Jobj0x0State {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(Jobj0x0StateInner {
                stream0_out: None,
                stream0: None,
            })),
        }
    }

    pub fn open(
        &self,
        stream0_out: stream::fd::FdStdio,
        stream0: stream::fd::FdCloser,
    ) -> Result<(), structure::ScriptExit> {
        let mut inner = self.inner.lock()?;

        // Closed check.
        if inner.stream0_out.is_some() {
            return Err("not closed".into());
        }
        if let Some(stream0) = inner.stream0.take() {
            stream0.stop()?;
        }

        // Setup
        inner.stream0_out.insert(stream0_out);
        inner.stream0.insert(stream0);

        Ok(())
    }

    pub fn close(&self) -> Result<(), structure::ScriptExit> {
        // If this can't lock it, exit early.
        let mut inner = self.inner.lock()?;

        let mut col = helpers::se_collect::ScriptExitCollector::new();

        if let Some(stream0) = inner.stream0.take() {
            // Don't error if it's already been taken / closed / never started.
            col.add_result(stream0.stop());
        }

        col.close_ok()
    }

    pub fn take_stream0_out(&self) -> Result<stream::fd::FdStdio, structure::ScriptExit> {
        let mut inner = self.inner.lock()?;
        match inner.stream0_out.take() {
            None => Err("already taken stream0_out".into()),
            Some(s) => Ok(s),
        }
    }
}

struct Jobj0x0StateInner {
    // cat module uses fd, so the lls.yaml declares a fd -> file stream.
    // There is no in, because that's the file.
    stream0_out: Option<stream::fd::FdStdio>,
    stream0: Option<stream::fd::FdCloser>,
}
