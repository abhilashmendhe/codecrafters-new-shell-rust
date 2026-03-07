#![allow(unused)]

use std::path::PathBuf;

use crate::{errors::MyShellError, intro::check_executable::check_executable};

#[derive(Debug)]
pub struct ShellFileType {
    pub f_name: String, 
    pub f_path: PathBuf,
    // pub output_format: String, 
    pub is_exe: bool
}

pub fn check_type(c_type: &str) -> Result<String, MyShellError> {
    match c_type {
        "echo" | "exit" | "type" | "pwd" | "cd" => Ok(format!("{} is a shell builtin", c_type)),
        _ => {
            // let mut output = format!("{}: not found", c_type);
            let shell_file_type = check_executable(c_type)?;
            if shell_file_type.is_exe {
                Ok(format!("{} is {}", c_type, shell_file_type.f_path.display()))
            } else {
                Ok(format!("{}: not found", c_type))
            }
        }
    }
}