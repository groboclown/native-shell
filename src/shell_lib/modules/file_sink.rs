//! The equivalent of '> filename'.

use std::sync::RwLock;
use std::{
    io::{self, Write},
    os::fd::OwnedFd,
};

use crate::shell_lib::helpers::log::Logger;
use crate::shell_lib::stream::fd::FdCloser;
use crate::shell_lib::structure::meta::{
    FixedStreamDef, JobModuleStruct, ModuleMeta, ModuleStreamStructure, ModuleStructure,
    NamedValue, StreamInterface, StreamType, ValueType,
};
use crate::shell_lib::structure::{ExecCtx, ExitCode, InitCtx, ScriptExit};
use crate::shell_lib::{helpers::abort_handler, structure::source::Source};

const BUFFER_SIZE: usize = 8192;

pub fn module_meta() -> ModuleMeta {
    ModuleMeta {
        name: "file_sink".to_string(),
        description: "Sink for file output".to_string(),
        version: "0.1.0".to_string(),
        authors: vec!["Native Shell Developers".to_string()],
        mod_name: vec![
            "shell_lib".to_string(),
            "modules".to_string(),
            "file_sink".to_string(),
        ],
        dependencies: vec![],
        os_dependencies: vec![],
        job: Some(JobModuleStruct {
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
                fields: vec![NamedValue {
                    name: "size".to_string(),
                    value_type: ValueType::Float,
                    optional: false,
                }],
            }),
            stream_struct: Some(ModuleStreamStructure {
                name: "FileSinkModuleStream".to_string(),
                fixed_streams: vec![FixedStreamDef {
                    name: Some("input".to_string()),
                    fd_index: Some(0),
                    stream_type: StreamType::Input(StreamInterface::FD),
                    required: true,
                }],
                input_variable: None,
                output_variable: None,
            }),
            handlers: vec![],
        }),
        command: None,
    }
}

pub struct FileSinkModule {
    state: abort_handler::RunState<FdCloser>,
    count: RwLock<f64>,
    source: Source,
    logger: Logger,
}

#[derive(Clone, Debug)]
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
    pub fn new(source: Source, ctx: &mut dyn InitCtx) -> Self {
        FileSinkModule {
            logger: Logger::new(&source, ctx),
            state: abort_handler::RunState::new(),
            count: RwLock::new(0.0),
            source,
        }
    }

    // The streams must be mut, as per the docs.
    pub fn exec(
        &self,
        ctx: &mut dyn ExecCtx,
        params: FileSinkModuleRuntimeParams,
        mut streams: FileSinkModuleStream,
    ) -> Result<(), ScriptExit> {
        self.logger
            .debug(ctx, format_args!("start write into {}", params.filename))?;
        let (inp, reader) = FdCloser::new_reader(streams.fd_0);
        self.state.start(inp);

        let mut ret: ExitCode = 0;
        if let Err(e) = self.exec_impl(&params, reader) {
            // Don't fail immediately; do that later.  Allow proper shutdown.
            let _ = self
                .logger
                .error(ctx, format_args!("{}: {}", params.filename, e));
            ret = 1;
        }

        // The FD close happens in the stop, in order ensure the
        // FD close happen just once.
        if let Err(e) = self.stop() {
            let _ = self.logger.error(
                ctx,
                format_args!(
                    "Failed to clean up file sink for {}: {:?}",
                    params.filename, e
                ),
            );
            ret = 1;
        }
        if ret != 0 { Err(ret.into()) } else { Ok(()) }
    }

    fn exec_impl<R: std::io::Read>(
        &self,
        params: &FileSinkModuleRuntimeParams,
        mut inp: R,
    ) -> Result<(), std::io::Error> {
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
                }
                Ok(size) => {
                    // Write the byte to the file.
                    // Errors mean stop.
                    out.write(&buf[..size])?;
                    out.flush()?;
                    *(self.count.write().unwrap()) += size as f64;
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => continue,
                Err(ref e) if e.kind() == io::ErrorKind::UnexpectedEof => {
                    // EOF reached, so stop.
                    return Ok(());
                }
                Err(e) => {
                    // If we get some other kind error, we should stop.
                    return Err(e);
                }
            }
        }
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
    fn stop(&self) -> Result<(), ScriptExit> {
        self.state.on_stop(|e| e.stop())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shell_lib::helpers::fd::{file_from_fd, mk_pipe, owned_from_file};
    use crate::shell_lib::internal::scheduler::eventbus::JobEventBuilder;
    use std::env;
    use std::sync::Arc;
    use std::{
        fs,
        io::{Read, Write},
        thread,
        time::Duration,
    };

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
        let mut evt_builder = JobEventBuilder::new();
        let input_file = fs::File::open(&input_path).unwrap();
        let fd = owned_from_file(input_file);
        let module = FileSinkModule::new(Source::default(), evt_builder.as_ctx());
        let out_path = dir.join("00_out.txt");
        let _ = fs::remove_file(&out_path);
        let params = FileSinkModuleRuntimeParams {
            filename: out_path.to_str().unwrap().to_string(),
            append: Some(false),
        };
        let streams = FileSinkModuleStream { fd_0: fd };

        let events = evt_builder.close().1;
        module
            .exec(events.as_serial_sender().as_ctx(), params, streams)
            .unwrap();
        // Verify output
        let mut contents = String::new();
        fs::File::open(&out_path)
            .unwrap()
            .read_to_string(&mut contents)
            .unwrap();
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
        let mut evt_builder = JobEventBuilder::new();
        let input_path = dir.join("01_in.txt");
        let _ = fs::remove_file(&input_path);
        {
            let mut f = fs::File::create(&input_path).unwrap();
            write!(f, "bar").unwrap();
        }
        let in_file = fs::File::open(&input_path).unwrap();
        let fd = owned_from_file(in_file);
        let module = FileSinkModule::new(Source::default(), evt_builder.as_ctx());

        let events = evt_builder.close().1;
        let params = FileSinkModuleRuntimeParams {
            filename: out_file.to_str().unwrap().to_string(),
            append: Some(true),
        };
        let streams = FileSinkModuleStream { fd_0: fd };
        module
            .exec(events.as_serial_sender().as_ctx(), params, streams)
            .unwrap();
        // Verify append
        let mut contents = String::new();
        fs::File::open(&out_file)
            .unwrap()
            .read_to_string(&mut contents)
            .unwrap();
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
        let mut evt_builder = JobEventBuilder::new();
        let input_path = dir.join("02_in.txt");
        let _ = fs::remove_file(&input_path);
        {
            let mut f = fs::File::create(&input_path).unwrap();
            write!(f, "bar").unwrap();
        }
        let in_file = fs::File::open(&input_path).unwrap();
        let fd = owned_from_file(in_file);
        let module = FileSinkModule::new(Source::default(), evt_builder.as_ctx());
        let params = FileSinkModuleRuntimeParams {
            filename: out_file.to_str().unwrap().to_string(),
            append: Some(true),
        };
        let streams = FileSinkModuleStream { fd_0: fd };

        let events = evt_builder.close().1;
        module
            .exec(events.as_serial_sender().as_ctx(), params, streams)
            .unwrap();
        // Verify append
        let mut contents = String::new();
        fs::File::open(&out_file)
            .unwrap()
            .read_to_string(&mut contents)
            .unwrap();
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
        let mut writer = file_from_fd(w);

        let mut evt_builder = JobEventBuilder::new();
        let params = FileSinkModuleRuntimeParams {
            filename: target.to_str().unwrap().to_string(),
            append: Some(false),
        };
        let streams = FileSinkModuleStream { fd_0: r };
        let module_arc = Arc::new(FileSinkModule::new(Source::default(), evt_builder.as_ctx()));
        let spawned = module_arc.clone();

        let events = evt_builder.close().1;
        let mut sender = events.as_serial_sender();
        let handle = thread::spawn(move || spawned.exec(sender.as_ctx(), params, streams).unwrap());

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
        handle.join().unwrap();

        // Verify output
        let mut contents = String::new();
        fs::File::open(&target)
            .unwrap()
            .read_to_string(&mut contents)
            .unwrap();
        assert!(contents.starts_with("dataxxxxx"));
        assert!(contents.len() == 4 + BUFFER_SIZE + 1);
    }
}
