//! The equivalent of '> filename'.

use std::{io::{self, Write}, os::fd::OwnedFd};
use std::sync::RwLock;

use crate::shell_lib::helpers::abort_handler;
use crate::shell_lib::{
    compile::meta::{
        FixedStreamDef, ModuleMeta, ModuleStreamStructure, ModuleStructure, NamedValue, StreamInterface, StreamType, ValueType
    },
    runtime::event_bus::{SendAction, ERROR_LOG},
};

const BUFFER_SIZE: usize = 8192;

pub fn module_meta() -> ModuleMeta {
    ModuleMeta {
        name: "file_sink".to_string(),
        description: "Sink for file output".to_string(),
        version: "0.1.0".to_string(),
        authors: vec!["Native Shell Developers".to_string()],
        mod_name: vec!["shell_lib".to_string(), "modules".to_string(), "file_sink".to_string()],
        dependencies: vec![],
        os_dependencies: vec![],
        instance_struct: "FileSinkModule".to_string(),
        compile_param_struct: None,
        runtime_param_struct: Some(ModuleStructure {
            name: "FileSinkModuleRuntimeParams".to_string(),
            new: None,
            fields: vec![
                NamedValue {
                    name: "filename".to_string(),
                    value_type: ValueType::String,
                    optional: false,
                },
                NamedValue {
                    name: "append".to_string(),
                    value_type: ValueType::Boolean,
                    optional: true,
                },
            ],
        }),
        state_struct: Some(ModuleStructure {
            name: "FileSinkModuleState".to_string(),
            new: None,
            fields: vec![
                NamedValue {
                    name: "size".to_string(),
                    value_type: ValueType::Float,
                    optional: false,
                },
            ],
        }),
        stream_struct: Some(ModuleStreamStructure {
            name: "FileSinkModuleStream".to_string(),
            fixed_streams: vec![
                FixedStreamDef {
                    name: Some("input".to_string()),
                    fd_index: Some(0),
                    stream_type: StreamType::Input(StreamInterface::Fd),
                    required: true,
                },
            ],
            input_variable: None,
            output_variable: None,
        }),
        handlers: vec![],
    }
}

pub struct FileSinkModule {
    state: abort_handler::RunState<abort_handler::FdIn>,
    count: RwLock<f64>,
}

pub struct FileSinkModuleRuntimeParams {
    pub filename: String,
    pub append: Option<bool>,
}

pub struct FileSinkModuleState {
    /// Bytes consumed by the file sink.
    pub size: f64,
}

pub struct FileSinkModuleStream {
    pub fd_0: OwnedFd,
}

impl FileSinkModule {
    pub fn new() -> Self {
        FileSinkModule { state: abort_handler::RunState::new(), count: RwLock::new(0.0) }
    }

    // The streams must be mut, as per the docs.
    pub fn exec(&self, params: FileSinkModuleRuntimeParams, mut streams: FileSinkModuleStream, alert: SendAction) -> Result<i16, String> {
        let (inp, reader) = abort_handler::FdIn::new(streams.fd_0);
        self.state.start(inp);

        let mut ret: i16 = 0;
        if let Err(e) = self.exec_impl(&params, reader) {
            alert(ERROR_LOG, format!("{}: {}", params.filename.clone(), e));
            ret = 1;
        }

        // The FD close happens in the stop, in order ensure the
        // FD close happen just once.
        if let Err(e) = self.stop() {
            alert(ERROR_LOG, format!("Failed to clean up file sink: {}", e));
        }
        Ok(ret)
    }


    fn exec_impl<R: std::io::Read>(&self, params: &FileSinkModuleRuntimeParams, mut inp: R) -> Result<(), std::io::Error> {
        {
            let mut count = self.count.write().unwrap();
            *count = 0.0;
        }
        let filename = params.filename.clone();
        let mut out = if params.append.unwrap_or(false) {
            std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(filename.clone())?
        } else {
            std::fs::File::create(filename.clone())?
        };

        let mut buf = [0 as u8; BUFFER_SIZE];
        loop {
            match inp.read(&mut buf) {
                Ok(size) if size == 0 => {
                    // No data and not would block (for non-blocking FD scenarios) means
                    // EOF.
                    return Ok(());
                },
                Ok(size) => {
                    // Write the byte to the file.
                    // Errors mean stop.
                    out.write(&buf[..size])?;
                    out.flush()?;
                    *(self.count.write().unwrap()) += size as f64;
                },
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => continue,
                Err(ref e) if e.kind() == io::ErrorKind::UnexpectedEof => {
                    // EOF reached, so stop.
                    return Ok(());
                },
                Err(e) => {
                    // If we get some other kind error, we should stop.
                    return Err(e);
                }
            }
        };
    }

    pub fn state(&self) -> FileSinkModuleState {
        FileSinkModuleState {
            size: *self.count.read().unwrap(),
        }
    }

    /// Required module abort handler.
    pub fn abort(&self) -> bool {
        let _ = self.stop();
        true
    }

    /// Stop the stream reading.
    fn stop(&self) -> Result<(), String> {
        self.state.on_stop(|e| e.stop())
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, io::{Read, Write}, thread, time::Duration};
    use std::env;
    use std::os::fd::OwnedFd;
    use std::sync::Arc;
    #[cfg(unix)]
    use std::os::unix::io::{FromRawFd, IntoRawFd};
    #[cfg(windows)]
    use std::os::windows::io::{FromRawHandle, IntoRawHandle};

    // Helper to wrap File into OwnedFd/OwnedHandle
    #[cfg(unix)]
    fn owned_from_file(file: fs::File) -> OwnedFd {
        let raw = file.into_raw_fd();
        unsafe { OwnedFd::from_raw_fd(raw) }
    }
    #[cfg(windows)]
    fn owned_from_file(file: fs::File) -> OwnedFd {
        let raw = file.into_raw_handle();
        unsafe { OwnedFd::from_raw_handle(raw) }
    }
    #[cfg(unix)]
    fn mk_pipe() -> (OwnedFd, OwnedFd) {
        use libc;
        use std::os::unix::io::FromRawFd;

        let mut fds = [0; 2];
        unsafe { if libc::pipe(fds.as_mut_ptr()) != 0 { panic!("pipe failed"); } }
        let r = unsafe { OwnedFd::from_raw_fd(fds[0]) };
        let w = unsafe { OwnedFd::from_raw_fd(fds[1]) };
        (r, w)
    }
    #[cfg(windows)]
    fn mk_pipe() -> (OwnedFd, OwnedFd) {
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

    // Note: because tests can run async, the file names must be unique.
    
    #[test]
    fn test_file_sink_writes_data_to_file() {
        // Prepare input file
        let dir = env::temp_dir();
        let input_path = dir.join("00_in.txt");
        let _ = fs::remove_file(&input_path);
        {
            let mut f = fs::File::create(&input_path).unwrap();
            write!(f, "hello").unwrap();
        }
        // Setup module and streams
        let input_file = fs::File::open(&input_path).unwrap();
        let fd = owned_from_file(input_file);
        let module = FileSinkModule::new();
        let out_path = dir.join("00_out.txt");
        let _ = fs::remove_file(&out_path);
        let params = FileSinkModuleRuntimeParams { filename: out_path.to_str().unwrap().to_string(), append: Some(false) };
        let streams = FileSinkModuleStream { fd_0: fd };
        let res = module.exec(params, streams, |g, m| { println!("00 {}: {}", g, m) }).unwrap();
        assert_eq!(res, 0);
        // Verify output
        let mut contents = String::new();
        fs::File::open(&out_path).unwrap().read_to_string(&mut contents).unwrap();
        assert_eq!(contents, "hello");
        let _ = fs::remove_file(&input_path);
        let _ = fs::remove_file(&out_path);
    }

    #[test]
    fn test_file_sink_appends_to_file() {
        let dir = env::temp_dir();
        let out_file = dir.join("01_out.txt");
        let _ = fs::remove_file(&out_file);
        {
            let mut f = fs::File::create(&out_file).unwrap();
            write!(f, "foo").unwrap();
        }
        // Append new content
        let input_path = dir.join("01_in.txt");
        let _ = fs::remove_file(&input_path);
        {
            let mut f = fs::File::create(&input_path).unwrap();
            write!(f, "bar").unwrap();
        }
        let in_file = fs::File::open(&input_path).unwrap();
        let fd = owned_from_file(in_file);
        let module = FileSinkModule::new();
        let params = FileSinkModuleRuntimeParams { filename: out_file.to_str().unwrap().to_string(), append: Some(true) };
        let streams = FileSinkModuleStream { fd_0: fd };
        let res = module.exec(params, streams, |g, m| { println!("01 {}: {}", g, m) }).unwrap();
        assert_eq!(res, 0);
        // Verify append
        let mut contents = String::new();
        fs::File::open(&out_file).unwrap().read_to_string(&mut contents).unwrap();
        assert_eq!(contents, "foobar");
        let _ = fs::remove_file(&out_file);
        let _ = fs::remove_file(&input_path);
    }

    #[test]
    fn test_file_sink_appends_to_non_existent_file() {
        let dir = env::temp_dir();
        let out_file = dir.join("02_out.txt");
        let _ = fs::remove_file(&out_file);
        // Append new content
        let input_path = dir.join("02_in.txt");
        let _ = fs::remove_file(&input_path);
        {
            let mut f = fs::File::create(&input_path).unwrap();
            write!(f, "bar").unwrap();
        }
        let in_file = fs::File::open(&input_path).unwrap();
        let fd = owned_from_file(in_file);
        let module = FileSinkModule::new();
        let params = FileSinkModuleRuntimeParams { filename: out_file.to_str().unwrap().to_string(), append: Some(true) };
        let streams = FileSinkModuleStream { fd_0: fd };
        let res = module.exec(params, streams, |g, m| { println!("02 {}: {}", g, m) }).unwrap();
        assert_eq!(res, 0);
        // Verify append
        let mut contents = String::new();
        fs::File::open(&out_file).unwrap().read_to_string(&mut contents).unwrap();
        assert_eq!(contents, "bar");
        let _ = fs::remove_file(&out_file);
        let _ = fs::remove_file(&input_path);
    }

    #[test]
    fn test_stop_async() {
        let dir = env::temp_dir();
        let target = dir.join("03_out.txt");

        // Create pipe
        let (r, w) = mk_pipe();
        let mut writer = unsafe { fs::File::from_raw_fd(w.into_raw_fd()) };

        let params = FileSinkModuleRuntimeParams { filename: target.to_str().unwrap().to_string(), append: Some(false) };
        let streams = FileSinkModuleStream { fd_0: r };
        let module_arc = Arc::new(FileSinkModule::new());
        let spawned = module_arc.clone();
        let handle = thread::spawn(move || {
            spawned.exec(params, streams, |g, m| { println!("03 {}: {}", g, m) }).unwrap()
        });

        // Write small data after exec started.
        write!(writer, "data").unwrap();
        writer.flush().unwrap();

        // Write beyond the buffer size to ensure it writes.
        let mut buf = [0; BUFFER_SIZE + 1];
        for i in 0..(BUFFER_SIZE + 1) {
            buf[i] = b'x';
        }
        writer.write_all(&buf).unwrap();
        writer.flush().unwrap();

        // Stop asynchronously
        thread::sleep(Duration::from_millis(10));
        module_arc.stop().unwrap();
        drop(writer);
        let res = handle.join().unwrap();
        assert_eq!(res, 0);

        // Verify output
        let mut contents = String::new();
        fs::File::open(&target).unwrap().read_to_string(&mut contents).unwrap();
        assert!(contents.starts_with("dataxxxxx"));
        assert!(contents.len() == 4 + BUFFER_SIZE + 1);
    }
}
