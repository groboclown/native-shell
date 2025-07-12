//! Methods for handling the abort calls into modules.

use std::{ops::DerefMut, os::fd::OwnedFd, sync::{Arc, RwLock}};
#[cfg(unix)]
use std::os::{fd::IntoRawFd, unix::io::RawFd};
#[cfg(unix)]
use libc;
#[cfg(windows)]
use std::os::windows::io::{AsRawHandle, RawHandle};


/// Manages the run state of a module.
/// Allows the module to handle abort cleaner.
pub struct RunState<T> {
    active: RwLock<Option<T>>,
}

pub type ActionHandler<T, R> = fn(&mut T) -> Result<R, String>;

impl<T> RunState<T> {
    pub fn new() -> Self {
        RunState {
            active: RwLock::new(None),
        }
    }

    /// Mark the module as active.
    pub fn start(&self, state: T) {
        let mut active_ref = self.active.write()
            .expect("Failed to lock the abort state mutex");
        active_ref.deref_mut().replace(state);
    }

    /// Check if the module is active.
    pub fn is_active(&self) -> bool {
        let active_ref = self.active.read()
            .expect("Failed to lock the abort state mutex");
        active_ref.as_ref().is_some()
    }

    /// Mark the module as stopped.
    /// If the module is not active, it will not run the handler.
    /// This will lock the state while the handler runs.
    pub fn on_stop(&self, handler: ActionHandler<T, ()>) -> Result<(), String> {
        let mut active_ref = self.active.write()
            .map_err(|e| format!("{}", e))?;
        {
            let state = active_ref.deref_mut();
            if (*state).is_some() {
                let mut s = (*state).take().unwrap();
                return handler(&mut s);
            }
        }
        Ok(())
    }

}

struct FdInState {
    open: super::state_guard::DeactivateGuard,
    #[cfg(unix)]
    fd: RawFd,
    #[cfg(windows)]
    fd: RawHandle,
}

impl FdInState {
    /// Close the file descriptor.
    pub fn close(&self) {
        // Implement the logic to close the FD here.
        if self.open.deactivate() {
            #[cfg(unix)]
            unsafe { libc::close(self.fd) };
            #[cfg(windows)]
            unsafe { libc::CloseHandle(self.fd) };
        }
    }

    pub fn close_on_fd_closed(&self) {
        self.open.deactivate();
    }

    pub fn is_closed(&self) -> bool {
        !self.open.is_active()
    }
}

impl Drop for FdInState {
    fn drop(&mut self) {
        self.close();
    }
}


/// A run state that owns an inbound FD.
/// These allow for the module to stop reads by closing the FD early.
/// For states that own multiple FDs, use a `RunState<Vec<FdReader>>`.
pub struct FdIn {
    state: Arc<FdInState>,
}

impl FdIn {
    /// Create a new `FdReader` from an owned file descriptor.
    pub fn new(fd: OwnedFd) -> (FdIn, FdReader) {
        #[cfg(unix)]
        let fd = fd.into_raw_fd();
        #[cfg(windows)]
        let fd = inp.into_raw_handle();

        let state = Arc::new(FdInState {
            open: super::state_guard::DeactivateGuard::new(),
            fd,
        });

        (FdIn{state: state.clone()}, FdReader { state })
    }

    pub fn stop(&self) -> Result<(), String> {
        self.state.close();
        Ok(())
    }
}


pub struct FdReader {
    state: Arc<FdInState>,
}

impl std::io::Read for FdReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.state.is_closed() {
            return Err(std::io::Error::new(std::io::ErrorKind::NotConnected, "Reader is closed"));
        }
        let ret = unsafe { libc::read(self.state.fd, buf.as_mut_ptr() as *mut _, buf.len()) };
        if ret == 0 {
            self.state.close();
            Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "EOF reached"))
        } else if ret < 0 {
            let err = std::io::Error::last_os_error();
            match err.raw_os_error() {
                Some(libc::EBADF) => {
                    // The FD is invalid.  Assume it's because this references an already closed FD.
                    // This can come from a bug on closing too soon.
                    self.state.close_on_fd_closed();
                    Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "EOF reached"))
                }
                Some(libc::EPIPE) => {
                    // The other end closed.
                    self.state.close();
                    Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "EOF reached"))
                }
                Some(_) => {
                    // Most likely due to the FD already closed.  To tell the state
                    // to then close the FD causes a panic.
                    self.state.close();
                    Err(err)
                }
                None => {
                    // Not an error, which is a bad state, but assume it's a closed FD.
                    self.state.close();
                    Err(err)
                }
            }
        } else {
            Ok(ret as usize)
        }
    }
}
