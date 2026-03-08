use std::io::Error;

use crate::{CMD, errors::MyShellError};

pub struct FileExists {
    pub message: String, 
    pub exists: bool
}

pub fn file_exists_check(cmd: CMD, fpath: &str) -> Result<FileExists, MyShellError> {

    if fpath.is_empty() {
        return Ok(FileExists { message: "".to_string(), exists: false });
    }

    let path = std::path::Path::new(&fpath);
    
    if path.exists() {
        Ok(FileExists { message: "".to_string(), exists: true })
    } else {
        let mut file_exists = FileExists { message: "".to_string(), exists: false };
        match cmd {
            CMD::CAT => {
                file_exists.message = format!("\ncat: {}: No such file or directory\n", fpath);
                Ok(file_exists)
            },
            CMD::LS => {
                file_exists.message = format!("\nls: cannot access '{}': No such file or directory\n", fpath);
                Ok(file_exists)
            },
            _ => Err(MyShellError::Io(Error::new(std::io::ErrorKind::NotFound, format!("No file not implemented for {:?}", cmd))))
        }
    }
} 