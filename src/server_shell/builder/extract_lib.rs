//! Extracts the library files from the embedded zip file into the output directory.

use std::include_bytes;

use crate::server_shell::builder::{errors::BuilderError, writer::SourceWriter};


pub fn extract_libs<SW: SourceWriter>(out: &SW) -> Result<(), BuilderError> {
    let zip_data = include_bytes!("../../../shell_lib.zip");
    let reader = std::io::Cursor::new(zip_data.as_slice());
    let mut archive = zip::ZipArchive::new(reader)
        .map_err(|e| BuilderError::ZipError(e))?;
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)
            .map_err(|e| BuilderError::ZipError(e))?;
        if file.is_dir() {
            continue;
        }
        let mut outfile = out.writer_for(file.name())
            .map_err(|e| BuilderError::IOError(e))?;
        std::io::copy(&mut file, &mut outfile)
            .map_err(|e| BuilderError::IOError(e))?;
    }
    Ok(())
}
