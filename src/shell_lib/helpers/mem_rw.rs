//! Read/Write implementation on top of a shared object.
//! Allows using in-memory read/write operations.

use std::io::{Read, Write};
use std::sync::mpsc;

pub struct MemRead {
    chan: mpsc::Receiver<Vec<u8>>,
}

pub struct MemWrite {
    chan: mpsc::Sender<Vec<u8>>,
}


/// Create a pair of in-memory read/write channels.
pub fn make_mem_read_write() -> (MemRead, MemWrite) {
    let (tx, rx) = mpsc::channel();
    (MemRead { chan: rx }, MemWrite { chan: tx })
}


impl Read for MemRead {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self.chan.recv() {
            Ok(data) => {
                let len = data.len().min(buf.len());
                buf[..len].copy_from_slice(&data[..len]);
                Ok(len)
            }
            Err(_) => Ok(0), // No more data
        }
    }
}

impl Write for MemWrite {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if self.chan.send(buf.to_vec()).is_err() {
            // EOF.
            return Err(std::io::Error::new(std::io::ErrorKind::BrokenPipe, "Channel closed"));
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
