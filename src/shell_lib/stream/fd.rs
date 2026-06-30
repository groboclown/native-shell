//SPDX:MIT

//! TODO fd stream needs rethought.

use libc;
#[cfg(not(windows))]
use std::os::fd::{FromRawFd as _, IntoRawFd as _, RawFd};
#[cfg(windows)]
use std::os::windows::io::{AsRawHandle, RawHandle};
use std::{fs::File, os::fd::OwnedFd, process::Stdio, sync::Arc};

use crate::shell_lib::{helpers::state_guard::DeactivateGuard, structure::ScriptExit};

/// An input file descriptor.  Built for the module streams.
pub struct FdIn {
    fd: OwnedFd,
}

impl FdIn {
    pub fn from_fd(fd: OwnedFd) -> Self {
        Self { fd }
    }

    /// Turn this File (which must be open in read mode) into an FdIn
    pub fn from_file(file: File) -> Self {
        Self { fd: file.into() }
    }

    /// Turn the FdIn into a reader, with io::Read semantics.
    pub fn into_reader(self) -> (FdCloser, FdReader) {
        FdCloser::new_reader(self.fd)
    }

    pub fn into_fd(self) -> (FdCloser, FdStdio) {
        FdCloser::new_fd(self.fd)
    }
}

/// An output file descriptor.  Built for the module streams.
pub struct FdOut {
    fd: OwnedFd,
}

impl FdOut {
    pub fn from_fd(fd: OwnedFd) -> Self {
        Self { fd }
    }

    /// Turn this File (which must be open in write mode) into an FdIn
    pub fn from_file(file: File) -> Self {
        Self { fd: file.into() }
    }

    pub fn into_writer(self) -> (FdCloser, FdWriter) {
        FdCloser::new_writer(self.fd)
    }

    pub fn into_fd(self) -> (FdCloser, FdStdio) {
        FdCloser::new_fd(self.fd)
    }
}

/// A run state that owns an inbound FD.
/// These allow for the module to stop reads by closing the FD early.
pub struct FdCloser {
    state: Arc<FdState>,
}

/// For states that own multiple FDs, use a `RunState<Vec<FdStream>>`.
impl FdCloser {
    /// Create a new 'FdStream' and `FdReader` from an owned file descriptor.
    pub fn new_reader(fd: OwnedFd) -> (FdCloser, FdReader) {
        let state = FdState::new(fd);
        (
            FdCloser {
                state: state.clone(),
            },
            FdReader { state },
        )
    }

    pub fn new_writer(fd: OwnedFd) -> (FdCloser, FdWriter) {
        let state = FdState::new(fd);
        (
            FdCloser {
                state: state.clone(),
            },
            FdWriter { state },
        )
    }

    pub fn new_fd(fd: OwnedFd) -> (FdCloser, FdStdio) {
        let state = FdState::new(fd);
        (
            FdCloser {
                state: state.clone(),
            },
            FdStdio { state },
        )
    }

    pub fn stop(&self) -> Result<(), ScriptExit> {
        self.state.close();
        Ok(())
    }
}

/// The reader, for modules that declare an input stream as a reader.
pub struct FdReader {
    state: Arc<FdState>,
}

impl std::io::Read for FdReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.state.is_closed() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotConnected,
                "Reader is closed",
            ));
        }
        let ret = unsafe { libc::read(self.state.fd, buf.as_mut_ptr() as *mut _, buf.len()) };
        if ret == 0 {
            self.state.close();
            Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "EOF reached",
            ))
        } else if ret < 0 {
            let err = std::io::Error::last_os_error();
            match err.raw_os_error() {
                Some(libc::EBADF) => {
                    // The FD is invalid.  Assume it's because this references an already closed FD.
                    // This can come from a bug on closing too soon.
                    self.state.close_on_fd_closed();
                    Err(std::io::Error::new(
                        std::io::ErrorKind::UnexpectedEof,
                        "EOF reached",
                    ))
                }
                Some(libc::EPIPE) => {
                    // The other end closed.
                    self.state.close();
                    Err(std::io::Error::new(
                        std::io::ErrorKind::UnexpectedEof,
                        "EOF reached",
                    ))
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

/// The writer, for modules that declare an output stream as a writer.
struct FdWriter {
    state: Arc<FdState>,
}

impl std::io::Write for FdWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let len: libc::size_t = buf.len().into();
        if len == 0 {
            return Ok(0);
        }
        if self.state.is_closed() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotConnected,
                "Writer is closed",
            ));
        }
        let ret = unsafe { libc::write(self.state.fd, buf.as_ptr() as *const libc::c_void, len) };
        if ret == 0 {
            self.state.close();
            Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "EOF reached",
            ))
        } else if ret < 0 {
            let err = std::io::Error::last_os_error();
            match err.raw_os_error() {
                Some(libc::EBADF) => {
                    // The FD is invalid.  Assume it's because this references an already closed FD.
                    // This can come from a bug on closing too soon.
                    self.state.close_on_fd_closed();
                    Err(std::io::Error::new(
                        std::io::ErrorKind::UnexpectedEof,
                        "EOF reached",
                    ))
                }
                Some(libc::EPIPE) => {
                    // The other end closed.
                    self.state.close();
                    Err(std::io::Error::new(
                        std::io::ErrorKind::UnexpectedEof,
                        "EOF reached",
                    ))
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

    fn flush(&mut self) -> std::io::Result<()> {
        if self.state.is_closed() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotConnected,
                "Writer is closed",
            ));
        }
        let ret = unsafe { libc::fsync(self.state.fd) };
        if ret == -1 {
            let err = std::io::Error::last_os_error();
            match err.raw_os_error() {
                Some(libc::EBADF) => {
                    // The FD is invalid.  Assume it's because this references an already closed FD.
                    // This can come from a bug on closing too soon.
                    self.state.close_on_fd_closed();
                    Err(std::io::Error::new(
                        std::io::ErrorKind::UnexpectedEof,
                        "EOF reached",
                    ))
                }
                Some(libc::EPIPE) => {
                    // The other end closed.
                    self.state.close();
                    Err(std::io::Error::new(
                        std::io::ErrorKind::UnexpectedEof,
                        "EOF reached",
                    ))
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
            Ok(())
        }
    }
}

/// A stdio + other file descriptor export handler structure.
/// Note that, due to its nature, the close status of the file descriptor
/// must be carefully handled.  Once it leaves this this instance through
/// an `.into()` call, the corresponding FdStream may not have its
/// closed state correctly in sync.  The FdStream will allow asynchronous,
/// immediate closing of the stream, though.
pub struct FdStdio {
    state: Arc<FdState>,
}

impl Into<std::io::Result<File>> for FdStdio {
    fn into(self) -> std::io::Result<File> {
        if self.state.is_closed() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotConnected,
                "fd is closed",
            ));
        }
        #[cfg(not(windows))]
        return Ok(unsafe { File::from_raw_fd(self.state.fd) });
        #[cfg(windows)]
        return Ok(unsafe { File::from_raw_handle(self.state.fd) });
    }
}

impl Into<std::io::Result<Stdio>> for FdStdio {
    fn into(self) -> std::io::Result<Stdio> {
        let file_res: std::io::Result<File> = self.into();
        let ret: Stdio = file_res?.into();
        Ok(ret)
    }
}

impl Into<std::io::Result<OwnedFd>> for FdStdio {
    fn into(self) -> std::io::Result<OwnedFd> {
        if self.state.is_closed() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotConnected,
                "fd is closed",
            ));
        }
        Ok(unsafe { OwnedFd::from_raw_fd(self.state.fd) })
    }
}

/// Underlying state for the file descriptor.
struct FdState {
    open: DeactivateGuard,
    #[cfg(unix)]
    fd: RawFd,
    #[cfg(windows)]
    fd: RawHandle,
}

impl FdState {
    pub fn new(fd: OwnedFd) -> Arc<Self> {
        #[cfg(unix)]
        let fd = fd.into_raw_fd();
        #[cfg(windows)]
        let fd = inp.into_raw_handle();

        Arc::new(FdState {
            open: DeactivateGuard::new(),
            fd,
        })
    }

    /// Close the file descriptor.
    pub fn close(&self) {
        if self.open.deactivate() {
            #[cfg(not(windows))]
            unsafe {
                libc::close(self.fd)
            };
            #[cfg(windows)]
            unsafe {
                libc::CloseHandle(self.fd)
            };
        }
    }

    pub fn close_on_fd_closed(&self) {
        self.open.deactivate();
    }

    pub fn is_closed(&self) -> bool {
        !self.open.is_active()
    }
}

impl Drop for FdState {
    fn drop(&mut self) {
        self.close();
    }
}
