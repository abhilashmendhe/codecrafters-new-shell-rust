#[allow(unused_imports)]
use std::io::{self, Write};

use crate::errors::MyShellError;

mod intro;
mod errors;

pub fn start_run() -> Result<(), MyShellError> {

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        io::stdin().read_line(&mut command)?;

        let u_command = command.trim();
        if u_command.eq("exit") {
            break;
        } else if u_command.starts_with("echo") {
            let res_string = &u_command[5..];
            println!("{}", res_string);
        } else {
            println!("{}: command not found", u_command);
        }
    }
    Ok(())
}