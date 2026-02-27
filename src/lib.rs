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

        let trim_cmd = command.trim();
        if trim_cmd.eq("exit") {
            break;
        } else if trim_cmd.starts_with("echo") {
            let ech_res_str = &trim_cmd[5..];
            println!("{}", ech_res_str);
        } else if trim_cmd.starts_with("type") {

            if trim_cmd.len() == 4 {
                continue;
            }
            let typ_res_str = &trim_cmd[5..];
            let res_str = check_type(typ_res_str)?;
            println!("{}", res_str);
        } else {
            println!("{}: command not found", trim_cmd);
        }
    }
    Ok(())
}