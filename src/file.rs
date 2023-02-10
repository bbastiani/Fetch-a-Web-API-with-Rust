use std::fs;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CheckFileError {
    #[error("File open error")]
    FileOpenError(#[from] std::io::Error),
}

// check if file exist or is empty
// return true if file exist and is empty
pub fn check_file(filepath: &str) -> Result<bool, CheckFileError> {
    if !Path::new(filepath).exists() {
        let _ = fs::File::create(filepath)?;
        return Ok(true);
    }
    let metadata = fs::metadata(filepath)?;
    Ok(metadata.len() == 0)
}