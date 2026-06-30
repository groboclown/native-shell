//! Read/Write implementation on top of a shared object.
//! Allows using in-memory read/write operations.

use std::io::{Error, ErrorKind, Read, Write};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::shell_lib::helpers::async_signal::{SignalNotice, SignalWait, signal};

/// Create a pair of in-memory read/write handlers.
/// They have no cap on the maximum write buffer length.
/// It allows for close announcements and non-blocking mode (by setting the timeout to zero).
pub fn make_mem_read_write(timeout: Duration) -> (MemRead, MemWrite) {
    let state = Arc::new(Mutex::new(MemState {
        data: None,
        open: true,
    }));
    let (send, recv) = signal();
    (
        MemRead {
            state: state.clone(),
            recv,
            send: send.clone(),
            timeout,
        },
        MemWrite { state, send },
    )
}

pub struct MemRead {
    state: Arc<Mutex<MemState>>,
    recv: SignalWait,
    send: SignalNotice,
    timeout: Duration,
}

impl MemRead {
    pub fn close(&mut self) -> std::io::Result<()> {
        MemState::close(&mut self.state, &self.send)
    }
}

pub struct MemWrite {
    state: Arc<Mutex<MemState>>,
    send: SignalNotice,
}

impl MemWrite {
    pub fn close(&mut self) -> std::io::Result<()> {
        MemState::close(&mut self.state, &self.send)
    }
}

struct MemState {
    data: Option<Vec<u8>>,
    open: bool,
}

impl MemState {
    pub fn close(state: &mut Arc<Mutex<MemState>>, send: &SignalNotice) -> std::io::Result<()> {
        let mut state = state
            .lock()
            .map_err(|_| Error::new(ErrorKind::Interrupted, "lock poisoned"))?;
        if state.open {
            state.open = false;
            state.data.take();
            send.notify();
        }
        Ok(())
    }

    pub fn read(&mut self, buf: &mut [u8]) -> std::io::Result<Option<usize>> {
        if self.open {
            match self.data.take() {
                None => Ok(None),
                Some(mut data) => {
                    let mut s_len = data.len();
                    let t_len = buf.len();
                    if t_len < s_len {
                        // There's more data in the source than what the buffer can take.
                        let remainder = data.split_off(t_len);
                        #[cfg(test)]
                        assert_eq!(t_len, data.len());
                        let _ = self.data.insert(remainder);
                        s_len = t_len;
                    } // else the target can consume the whole array.
                    buf[..s_len].copy_from_slice(data.as_slice());
                    Ok(Some(s_len))
                }
            }
        } else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::BrokenPipe,
                "Channel closed",
            ));
        }
    }

    pub fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if self.open {
            let len = buf.len();
            match self.data.take() {
                None => {
                    let mut v = Vec::with_capacity(len);
                    v.extend_from_slice(buf);
                    self.data.insert(v);
                }
                Some(mut data) => {
                    data.extend_from_slice(buf);
                    self.data.insert(data);
                }
            }
            Ok(len)
        } else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::BrokenPipe,
                "Channel closed",
            ));
        }
    }
}

impl Read for MemRead {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        loop {
            {
                let mut state = self
                    .state
                    .lock()
                    .map_err(|_| Error::new(ErrorKind::Interrupted, "lock poisoned"))?;
                if let Some(v) = state.read(buf)? {
                    return Ok(v);
                }
            }
            if self.timeout <= Duration::ZERO || !self.recv.wait_for(self.timeout) {
                return Ok(0);
            }
        }
    }
}

impl Write for MemWrite {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut state = self
            .state
            .lock()
            .map_err(|_| Error::new(ErrorKind::Interrupted, "lock poisoned"))?;
        state.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
