use crate::{errors::MyShellError, intro::read_type_path::read_type_path};

pub fn check_type(c_type: &str) -> Result<String, MyShellError> {
    match c_type {
        "echo" | "exit" | "type" | "pwd" | "cd" => Ok(format!("{} is a shell builtin", c_type)),
        _ => {
            read_type_path(c_type)
        }
    }
}