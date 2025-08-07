//! Abstraction and implementation of writing sources.

use std::path::Path;

pub trait SourceWriter {
    fn writer_for(&self, file_name: &str) -> Result<Box<dyn std::io::Write>, std::io::Error>;
}

pub struct FileSourceWriter {
    pub base_path: String,
}

impl SourceWriter for FileSourceWriter {
    fn writer_for(&self, file_name: &str) -> Result<Box<dyn std::io::Write>, std::io::Error> {
        let full_path = format!("{}/{}", self.base_path, file_name);
        let full_path = Path::new(full_path.as_str());
        if let Some(parent) = full_path.parent() {
            if !parent.exists() {
                std::fs::create_dir(parent)?;
            }
        }

        let file = std::fs::File::create(full_path)?;
        Ok(Box::new(file))
    }
}

impl FileSourceWriter {
    pub fn new(base_path: &str) -> Result<Self, std::io::Error> {
        std::fs::create_dir_all(base_path)?;
        Ok(FileSourceWriter {
            base_path: base_path.to_string(),
        })
    }
}
