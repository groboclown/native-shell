//! Helps with using file descriptors for the node streams.

use std::io::Write;
#[cfg(unix)]
use std::os::fd::OwnedFd;
#[cfg(unix)]
use std::os::unix::io::{FromRawFd, IntoRawFd};
#[cfg(windows)]
use std::os::windows::io::{FromRawHandle, IntoRawHandle};
use std::sync::{Arc, RwLock};


/// Create a pair of file descriptors that come from OS pipe objects, in the form (read, write).
#[cfg(unix)]
pub fn mk_pipe() -> (OwnedFd, OwnedFd) {
    use libc;
    use std::os::unix::io::FromRawFd;

    let mut fds = [0; 2];
    unsafe { if libc::pipe(fds.as_mut_ptr()) != 0 { panic!("pipe failed"); } }
    let r = unsafe { OwnedFd::from_raw_fd(fds[0]) };
    let w = unsafe { OwnedFd::from_raw_fd(fds[1]) };
    (r, w)
}
#[cfg(windows)]
pub fn mk_pipe() -> (OwnedFd, OwnedFd) {
    use std::ptr::null_mut;
    use std::os::windows::io::FromRawHandle;
    use winapi::um::namedpipeapi::CreatePipe;

    let mut read_pipe = null_mut();
    let mut write_pipe = null_mut();
    unsafe {
        if CreatePipe(&mut read_pipe, &mut write_pipe, null_mut(), 0) == 0 {
            panic!("CreatePipe failed");
        }
        let r = OwnedFd::from_raw_handle(read_pipe);
        let w = OwnedFd::from_raw_handle(write_pipe);
        (r, w)
    }
}

/// Turn a File into OwnedFd.
#[cfg(unix)]
pub fn owned_from_file(file: std::fs::File) -> OwnedFd {
    let raw = file.into_raw_fd();
    unsafe { OwnedFd::from_raw_fd(raw) }
}
#[cfg(windows)]
pub fn owned_from_file(file: std::fs::File) -> OwnedFd {
    let raw = file.into_raw_handle();
    unsafe { OwnedFd::from_raw_handle(raw) }
}

/// Get a File from an OwnedFd.
/// The caller must ensure the File follows the read or write semantics based on the FD.
pub fn file_from_fd(fd: OwnedFd) -> std::fs::File {
    #[cfg(unix)]
    return unsafe { std::fs::File::from_raw_fd(fd.into_raw_fd()) };
    #[cfg(windows)]
    return unsafe { std::fs::File::from_raw_handle(fd.into_raw_handle()) };
}

// The following are unit test helpers.

/// Create a FD reader from the given data, which can be used in tests.
pub fn make_fd_reader(data: &[u8]) -> OwnedFd {
    let (r, w) = mk_pipe();
    let mut writer = file_from_fd(w);
    writer.write_all(data).unwrap();
    // close writer to send EOF
    drop(writer);
    r
}

/// Writes to a vector, which can be monitored by the tests.
pub struct VecWriter {
    data: Arc<RwLock<Vec<u8>>>,
}

impl VecWriter {
    /// Create a new VecWriter with the given data, wrapped in a box.
    pub fn new_box(data: Arc<RwLock<Vec<u8>>>) -> Box<Self> {
        Box::new(VecWriter { data })
    }

    /// Create a new Box VecWriter 
    pub fn new_pair() -> (Box<Self>, Arc<RwLock<Vec<u8>>>) {
        let data = Arc::new(RwLock::new(Vec::new()));
        (Self::new_box(data.clone()), data)
    }
}

impl Write for VecWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.data.write().unwrap().extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
