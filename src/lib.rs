#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::Command;

use crate::{errors::MyShellError, intro::{check_executable::check_executable, check_type::check_type}, navigation::go_to_path::go_to_path, quoting::ech_extract};

pub mod errors;
mod intro;
mod navigation; 
mod quoting;

pub enum CMD {
    ECHO,
    CAT,
    EXEC
}

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

            let stmt = trim_cmd[4..].trim_ascii_start();
            let res_str = ech_extract(stmt, CMD::ECHO)?;
            println!("{}", res_str);
        } else if trim_cmd.starts_with("cat") {

            let stmt = trim_cmd[3..].trim_ascii_start();
            // let ret_files = ech_extract(stmt, CMD::ECHO)?;
            let res_str = ech_extract(stmt, CMD::CAT)?;
            println!("{}", res_str);
            // println!("{}",ret_files);
        } else if trim_cmd.starts_with("type") {

            if trim_cmd.len() == 4 {
                continue;
            }
            let typ_res_str = &trim_cmd[5..];
            let res_str = check_type(typ_res_str)?;
            println!("{}", res_str);
        } else if trim_cmd.eq("pwd") {

            let pwd = std::env::current_dir()?;
            println!("{}", pwd.display());
        } else if trim_cmd.starts_with("cd") {
            
            let res_str = go_to_path(&trim_cmd[2..])?;
            if res_str.len() > 0 {
                println!("{}", res_str);
            }
        } else {
            // println!("trim_cmd: {}", trim_cmd);
            let output = ech_extract(trim_cmd, CMD::EXEC)?;
            // println!("{}", output);
            let spl_cmds = output.split("|").collect::<Vec<&str>>();
            
            // println!("START src/lib.rs 'else' condition");
            // println!("{}", spl_cmds[0]);
            let shell_f_type = check_executable(spl_cmds[0])?;
            if shell_f_type.is_exe {
                let script_out = Command::new(&spl_cmds[0])
                    .args(&spl_cmds[1..])
                    .output()?;
                print!("{}", String::from_utf8_lossy(&script_out.stdout));
            } else {
                println!("{}: command not found", &spl_cmds[0]);
            }
            // println!("START src/lib.rs 'else' END");
        }
    }
    Ok(())
}