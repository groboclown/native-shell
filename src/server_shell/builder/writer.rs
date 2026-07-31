//! Abstraction and implementation of writing sources.

use std::{
    collections::HashMap,
    io,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use super::errors;

/// Creates writers for files under the source.
pub trait SourceWriter {
    /// Get a writer for the relative path.
    /// This returns an error if another request already happened for the same filename.
    fn writer_for<'a, 'b>(
        &'a self,
        file_name: &'b Path,
    ) -> Result<Box<dyn io::Write>, errors::BuilderError>;
}

/// Writes sources to a file system under a base path.
#[derive(Clone)]
pub struct FileSourceWriter {
    pub base_path: Arc<PathBuf>,
}

impl SourceWriter for FileSourceWriter {
    fn writer_for<'a, 'b>(
        &'a self,
        file_name: &'b Path,
    ) -> Result<Box<dyn io::Write>, errors::BuilderError> {
        let mut full_path = self.base_path.as_ref().clone();
        full_path.push(file_name);
        if let Some(parent) = full_path.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent).map_err(|e| errors::file_err(parent, e))?;
            }
        }

        let file = std::fs::File::create(full_path.as_path())?;
        Ok(Box::new(file))
    }
}

impl FileSourceWriter {
    pub fn new<'a, T: Into<&'a Path>>(base_path: T) -> Result<Arc<Self>, errors::BuilderError> {
        let p: &'a Path = base_path.into();
        std::fs::create_dir_all(p).map_err(|e| errors::file_err(p, e))?;
        Ok(Arc::new(FileSourceWriter {
            base_path: Arc::new(PathBuf::from(p)),
        }))
    }
}

/// An in-memory source writer.
/// Useful for tests.
#[derive(Clone)]
pub(crate) struct MemSourceWriter {
    pub files: Arc<Mutex<HashMap<String, Vec<u8>>>>,
}

impl MemSourceWriter {
    pub fn new() -> Self {
        Self {
            files: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn get_for(&self, name: &str) -> Option<Vec<u8>> {
        match self.files.lock() {
            Ok(m) => m.get(&name.to_string()).map(|v| v.clone()),
            Err(e) => (*e.get_ref()).get(name).map(|v| v.clone()),
        }
    }
}

impl SourceWriter for MemSourceWriter {
    fn writer_for<'a, 'b>(
        &'a self,
        file_name: &'b Path,
    ) -> Result<Box<dyn io::Write>, errors::BuilderError> {
        let file_name: &'b Path = file_name.into();
        let file_name = file_name.as_os_str().to_string_lossy().to_string();
        let exists = match self.files.lock() {
            Ok(m) => m.contains_key(&file_name),
            Err(e) => e.get_ref().contains_key(&file_name),
        };
        if exists {
            Err(errors::file_err(
                file_name.clone(),
                io::Error::new(io::ErrorKind::AlreadyExists, file_name),
            ))
        } else {
            Ok(Box::new(MemPathWriter {
                file_name: file_name,
                files: self.files.clone(),
            }))
        }
    }
}

struct MemPathWriter {
    file_name: String,
    files: Arc<Mutex<HashMap<String, Vec<u8>>>>,
}

impl io::Write for MemPathWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self.files.lock() {
            Ok(mut m) => append_to(&mut m, &self.file_name, buf),
            Err(mut e) => append_to(e.get_mut(), &self.file_name, buf),
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

fn append_to<'a, 'b>(map: &'a mut HashMap<String, Vec<u8>>, entry: &String, data: &'b [u8]) {
    match map.get_mut(entry) {
        Some(b) => b.extend_from_slice(data),
        None => {
            map.insert(entry.clone(), data.into());
        }
    }
}
