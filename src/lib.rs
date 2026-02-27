#[allow(unused_imports)]
use std::io::{self, Write};

use crate::{errors::MyShellError, intro::check_type::check_type};

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
            let ech_res_str = &u_command[5..];
            println!("{}", ech_res_str);
        } else if u_command.starts_with("type") {
            let typ_res_str = &u_command[5..];
            let res_str = check_type(typ_res_str);
            println!("{}", res_str);
        } else {
            println!("{}: command not found", u_command);
        }
    }
    Ok(())
}