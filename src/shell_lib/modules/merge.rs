//! Merge 0 or more input streams into a single output stream.
//! Allows for line-based or other separators to use as the merge delimiter.
//! This performs byte-wise merging, so it is suitable for UTF-8 and other text formats
//! but ONLY if the separator is a valid UTF-8 sequence.

use std::{collections::VecDeque, io::Read, os::fd::OwnedFd};

use crate::shell_lib::{compile::{job, meta}, helpers::abort_handler};
use crate::shell_lib::runtime::event_bus::ERROR_LOG;

const BUFFER_SIZE: f64 = 8192.0;

pub fn module_meta() -> meta::ModuleMeta {
    meta::ModuleMeta {
        name: "merge".to_string(),
        description: "Merge multiple input streams into one output stream".to_string(),
        version: "0.1.0".to_string(),
        authors: vec!["Native Shell Developers".to_string()],
        mod_name: vec!["shell_lib".to_string(), "modules".to_string(), "merge".to_string()],
        dependencies: vec![],
        os_dependencies: vec![],
        instance_struct: "MergeModule".to_string(),
        compile_param_struct: None,
        runtime_param_struct: Some(meta::ModuleStructure {
            name: "MergeModuleRuntimeParams".to_string(),
            new: None,
            fields: vec![
                meta::NamedValue {
                    name: "separator".to_string(),
                    value_type: meta::ValueType::String,
                    optional: true,
                },
                meta::NamedValue {
                    name: "max_record_length".to_string(),
                    value_type: meta::ValueType::Float,
                    optional: true,
                },
                meta::NamedValue {
                    name: "stream_prefix".to_string(),
                    value_type: meta::ValueType::StringList,
                    optional: false,
                },
            ],
        }),
        state_struct: None,
        stream_struct: Some(meta::ModuleStreamStructure {
            name: "MergeModuleStream".to_string(),
            fixed_streams: vec![
                meta::FixedStreamDef {
                    name: Some("output".to_string()),
                    fd_index: Some(0),
                    stream_type: meta::StreamType::Output(meta::StreamInterface::ReadWrite),
                    required: true,
                },
            ],
            input_variable: Some(meta::VariableStreamField {
                field_name: "input".to_string(),
                stream_type: meta::StreamInterface::Fd,
                min_count: 0,
                max_count: u16::MAX,
            }),
            output_variable: None,
        }),
        handlers: vec![],
    }
}

#[derive(Clone, Debug)]
pub struct MergeModuleRuntimeParams {
    pub separator: Option<String>,
    pub max_record_length: Option<f64>,
    pub stream_prefix: Option<Vec<String>>,
}

pub struct MergeModuleStream {
    pub fd_0: Box<dyn std::io::Write + Send>,
    pub input: Vec<OwnedFd>,
}

pub struct MergeModule {
    state: abort_handler::RunState<Vec<abort_handler::FdIn>>,
}

impl MergeModule {
    pub fn new() -> Self {
        MergeModule { state: abort_handler::RunState::new() }
    }

    pub fn exec(&self, context: Box<dyn job::JobRunnerContext>, params: MergeModuleRuntimeParams, mut streams: MergeModuleStream) -> Result<job::ExitCode, String> {
        let separator = match params.separator {
            Some(s) => s,
            None => "\n".to_string(),
        };
        let separator = separator.as_str();
        let max_record_length = params.max_record_length.unwrap_or(BUFFER_SIZE) as usize;
        let mut active = VecDeque::with_capacity(streams.input.len());
        {
            let stream_prefix = params.stream_prefix.unwrap_or_default();
            let mut stream_prefix = stream_prefix.iter();
            let mut state = Vec::with_capacity(streams.input.len());
            for fd in streams.input {
                let (inp, reader) = abort_handler::FdIn::new(fd);
                active.push_back(BuffFd::new(reader, max_record_length, stream_prefix.next().unwrap_or(&String::new()).clone()));
                state.push(inp);
            }
            self.state.start(state);
        }

        // Read blocks of text, split by the separator, and write to the output stream.
        // This loops over each input stream, one block at a time, until all input streams are exhausted.
        while active.len() > 0 {
            let mut valid_inp = VecDeque::with_capacity(active.len());

            loop {
                let mut inp = match active.pop_front() {
                    Some(inp) => inp,
                    None => break,
                };

                let data = inp.read_until(separator).map_err(|e| {
                    let _ = (*context).send_event(ERROR_LOG.to_string(), job::EventPayload::Message(format!("Failed to read from input stream: {}", e)));
                    e.to_string()
                })?;
                let prefix = inp.prefix.clone();
                let prefix = prefix.as_bytes();
                if inp.is_open() {
                    // If the input stream is still open, push it back to the active list.
                    valid_inp.push_back(inp);
                }
                if !data.is_empty() {
                    match streams.fd_0.write_all(prefix) {
                        Ok(_) => {
                            // If we successfully wrote to the output stream, continue reading from this input stream.
                        }
                        Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                            // EOF means cannot continue writing, so stop immediately.
                            let _ = self.stop();
                            return Ok(0);
                        }
                        Err(e) if e.kind() == std::io::ErrorKind::Interrupted || e.kind() == std::io::ErrorKind::WouldBlock => {
                            // This is bad.  It means a retry is needed, which makes this logic more complex.
                            // Not supported at the moment.
                            let _ = (*context).send_event(ERROR_LOG.to_string(), job::EventPayload::Message(format!("Failed to write to output stream: {}", e)));
                            let _ = self.stop();
                            return Ok(2);
                        }
                        Err(e) => {
                            let _ = (*context).send_event(ERROR_LOG.to_string(), job::EventPayload::Message(format!("Failed to write to output stream: {}", e)));
                            let _ = self.stop();
                            return Ok(1);
                        }
                    }
                    match streams.fd_0.write_all(data.as_slice()) {
                        Ok(_) => {
                            // If we successfully wrote to the output stream, continue reading from this input stream.
                        }
                        Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                            // EOF means cannot continue writing, so stop immediately.
                            let _ = self.stop();
                            return Ok(0);
                        }
                        Err(e) if e.kind() == std::io::ErrorKind::Interrupted || e.kind() == std::io::ErrorKind::WouldBlock => {
                            // This is bad.  It means a retry is needed, which makes this logic more complex.
                            // Not supported at the moment.
                            let _ = (*context).send_event(ERROR_LOG.to_string(), job::EventPayload::Message(format!("Failed to write to output stream: {}", e)));
                            let _ = self.stop();
                            return Ok(2);
                        }
                        Err(e) => {
                            let _ = (*context).send_event(ERROR_LOG.to_string(), job::EventPayload::Message(format!("Failed to write to output stream: {}", e)));
                            let _ = self.stop();
                            return Ok(1);
                        }
                    }
                }
            }

            active = valid_inp;
        }
        // The FD close happens in the stop, in order ensure the
        // FD close happen just once.
        if let Err(e) = self.stop() {
            let _ = (*context).send_event(ERROR_LOG.to_string(), job::EventPayload::Message(format!("Failed to clean up file sink: {}", e)));
        }
        Ok(0)
    }


    /// Required module abort handler.
    pub fn abort(&self) -> bool {
        let _ = self.stop();
        true
    }

    /// Stop the stream reading.
    fn stop(&self) -> Result<(), String> {
        self.state.on_stop(|e| {
            e.iter().for_each(|e| { let _ = e.stop(); });
            Ok(())
        })
    }
}

pub struct BuffFd {
    buf: Vec<u8>,
    filled: usize,
    capacity: usize,
    inp: abort_handler::FdReader,
    open: bool,
    pub prefix: String,
}

impl BuffFd {
    pub fn new(reader: abort_handler::FdReader, capacity: usize, prefix: String) -> Self {
        let mut buf = Vec::with_capacity(capacity);
        buf.resize(capacity, 0);
        Self { buf, filled: 0, capacity, inp: reader, open: true, prefix }
    }

    pub fn is_open(&self) -> bool {
        self.open || self.filled > 0
    }

    pub fn read_until(&mut self, delim: &str) -> Result<Vec<u8>, std::io::Error> {
        if self.open {
            let count = match self.inp.read(&mut self.buf.as_mut_slice()[self.filled..self.capacity]) {
                Ok(0) => {
                    // EOF reached, return the data read so far.
                    self.open = false;
                    0
                }
                Ok(count) => count,
                Err(e) if e.kind() == std::io::ErrorKind::Interrupted || e.kind() == std::io::ErrorKind::WouldBlock => {
                    // Skip this read, as it would block.
                    0
                }
                Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                    // EOF reached, return the data read so far.
                    self.open = false;
                    0
                }
                Err(e) => return Err(e),
            };
            self.filled += count;
        }
        if self.filled == 0 {
            return Ok(vec![]);
        }
        // Handle empty delimiter: return all buffered data
        if delim.is_empty() {
            let data = self.buf[..self.filled].to_vec();
            self.filled = 0;
            return Ok(data);
        }
        match self.find(delim) {
            Some(pos) => {
                // We found the delimiter, return the data up to the delimiter.
                let data = self.buf[..pos + delim.len()].to_vec();
                self.buf.copy_within(pos + delim.len()..self.filled, 0);
                self.filled -= pos + delim.len();
                Ok(data)
            }
            None => {
                // No delimiter found.  The behavior in this case depends on the capacity and the open state.
                if self.filled == self.capacity || !self.open {
                    // Return what's left in the buffer.
                    let data = self.buf[..self.filled].to_vec();
                    self.filled = 0;
                    Ok(data)
                } else {
                    Ok(vec![])
                }
            }
        }
    }

    fn find(&self, delim: &str) -> Option<usize> {
        let slice = std::str::from_utf8(&self.buf.as_slice()[0..self.filled]).ok()?;
        slice.find(delim)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{cell::RefCell, sync::Arc};
    use std::io::Write;
    use std::thread;
    use std::time::Duration;
    use crate::shell_lib::helpers::fd::{file_from_fd, mk_pipe, make_fd_reader, VecWriter};

    struct EventBus {
        name: &'static str,
        msgs: RefCell<Vec<(String, job::EventPayload)>>,
    }
    impl job::JobRunnerContext for EventBus {
        fn send_event(&self, event_ref: job::EventRef, payload: job::EventPayload) -> Result<(), String> {
            println!("{} {}: {}", self.name, event_ref, payload);
            self.msgs.borrow_mut().push((event_ref, payload));
            Ok(())
        }
    }

    #[test]
    fn test_zero_inputs() {
        let bus = EventBus { name: "test_zero_inputs", msgs: RefCell::new(vec![]) };
        let module = MergeModule::new();
        let (fd_0, output) = VecWriter::new_pair();
        let streams = MergeModuleStream { fd_0, input: vec![] };
        let code = module.exec(
            Box::new(bus),
            MergeModuleRuntimeParams { separator: None, max_record_length: None, stream_prefix: None },
            streams,
        ).unwrap();
        assert_eq!(code, 0);
        assert!(output.read().unwrap().is_empty());
    }

    #[test]
    fn test_single_input_default_separator() {
        let bus = EventBus { name: "test_single_input_default_separator", msgs: RefCell::new(vec![]) };
        let module = MergeModule::new();
        let (fd_0, output) = VecWriter::new_pair();
        let reader = make_fd_reader(b"hello\nworld\n");
        let streams = MergeModuleStream { fd_0, input: vec![reader] };
        let code = module.exec(
            Box::new(bus),
            MergeModuleRuntimeParams { separator: None, max_record_length: None, stream_prefix: None },
            streams,
        ).unwrap();
        assert_eq!(code, 0);
        assert_eq!(*output.read().unwrap(), b"hello\nworld\n");
    }

    #[test]
    fn test_two_inputs_default_separator() {
        let bus = EventBus { name: "test_single_input_default_separator", msgs: RefCell::new(vec![]) };
        let module = MergeModule::new();
        let (fd_0, output) = VecWriter::new_pair();
        let a = make_fd_reader(b"a1\na2\na3");
        let b = make_fd_reader(b"b1\nb2\n");
        let streams = MergeModuleStream { fd_0, input: vec![a, b] };
        let code = module.exec(
            Box::new(bus),
            MergeModuleRuntimeParams { separator: None, max_record_length: None, stream_prefix: None },
            streams,
        ).unwrap();
        assert_eq!(code, 0);
        assert_eq!(*output.read().unwrap(), b"a1\nb1\na2\nb2\na3");
    }

    #[test]
    fn test_two_inputs_prefix() {
        let bus = EventBus { name: "test_single_input_default_separator", msgs: RefCell::new(vec![]) };
        let module = MergeModule::new();
        let (fd_0, output) = VecWriter::new_pair();
        let a = make_fd_reader(b"a1\na2\na3");
        let b = make_fd_reader(b"b1\nb2\n");
        let streams = MergeModuleStream { fd_0, input: vec![a, b] };
        let code = module.exec(
            Box::new(bus),
            MergeModuleRuntimeParams { separator: None, max_record_length: None, stream_prefix: Some(vec!["1:".to_string(), "2:".to_string()]) },
            streams,
        ).unwrap();
        assert_eq!(code, 0);
        assert_eq!(*output.read().unwrap(), b"1:a1\n2:b1\n1:a2\n2:b2\n1:a3");
    }

    #[test]
    fn test_two_inputs_separator_two_chars() {
        let bus = EventBus { name: "test_two_inputs_separator_two_chars", msgs: RefCell::new(vec![]) };
        let module = MergeModule::new();
        let (fd_0, output) = VecWriter::new_pair();
        let a = make_fd_reader(b"a1||a2||");
        let b = make_fd_reader(b"b1||b2||");
        let streams = MergeModuleStream { fd_0, input: vec![a, b] };
        let code = module.exec(
            Box::new(bus),
            MergeModuleRuntimeParams { separator: Some("||".to_string()), max_record_length: None, stream_prefix: None },
            streams,
        ).unwrap();
        assert_eq!(code, 0);
        // Expect interleaved reads: a1, b1, a2, b2
        assert_eq!(*output.read().unwrap(), b"a1||b1||a2||b2||");
    }

    #[test]
    fn test_shorter_than_separator() {
        let bus = EventBus { name: "test_shorter_than_separator", msgs: RefCell::new(vec![]) };
        let module = MergeModule::new();
        let (fd_0, output) = VecWriter::new_pair();
        let reader = make_fd_reader(b"xyz");
        let streams = MergeModuleStream { fd_0, input: vec![reader] };
        let code = module.exec(
            Box::new(bus),
            MergeModuleRuntimeParams { separator: Some("abcd".to_string()), max_record_length: None, stream_prefix: None },
            streams,
        ).unwrap();
        assert_eq!(code, 0);
        assert_eq!(*output.read().unwrap(), b"xyz");
    }

    #[test]
    fn test_longer_than_max_record() {
        let bus = EventBus { name: "test_longer_than_max_record", msgs: RefCell::new(vec![]) };
        let module = MergeModule::new();
        let (fd_0, output) = VecWriter::new_pair();
        let data = b"1234567\n";
        let reader = make_fd_reader(data);
        let streams = MergeModuleStream { fd_0, input: vec![reader] };
        let code = module.exec(
            Box::new(bus),
            MergeModuleRuntimeParams { separator: Some("\n".to_string()), max_record_length: Some(3.0), stream_prefix: None },
            streams,
        ).unwrap();
        assert_eq!(code, 0);
        assert_eq!(*output.read().unwrap(), b"1234567\n");
    }

    #[test]
    fn test_standalone_abort_method() {
        let module = MergeModule::new();
        assert!(module.abort());
    }

    #[test]
    fn test_exec_stops_when_aborted_midstream() {
        // Set up a MergeModule and a pipe reader that will block on read()
        let bus = EventBus { name: "test_abort_mid_exec", msgs: RefCell::new(vec![]) };
        let module = Arc::new(MergeModule::new());
        let (fd_0, output) = VecWriter::new_pair();
        let (reader_fd, writer_fd) = mk_pipe();
        let mut writer = file_from_fd(writer_fd);
        let data = b"abc\ndef";
        writer.write_all(data).unwrap();
        let streams = MergeModuleStream { fd_0, input: vec![reader_fd] };

        // Spawn the exec in a separate thread; it will block waiting for input
        let module_clone = module.clone();
        let handle = thread::spawn(move || {
            module_clone.exec(
                Box::new(bus),
                MergeModuleRuntimeParams { separator: None, max_record_length: None, stream_prefix: None },
                streams
            ).unwrap()
        });

        // Give exec time to start and block on read
        thread::sleep(Duration::from_millis(50));
        // Trigger abort, then close the writer end to unblock read()
        assert!(module.abort());
        drop(writer);

        // Join the thread and verify exec returned success and wrote nothing
        let code = handle.join().unwrap();
        assert_eq!(code, 0);
        assert_eq!(*output.read().unwrap(), b"abc\ndef");
    }
}
