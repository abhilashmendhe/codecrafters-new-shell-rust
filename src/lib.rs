#[allow(unused_imports)]
use std::io::{self, Write};

use crate::errors::MyShellError;

mod intro;
mod errors;

pub fn start_run() -> Result<(), MyShellError> {
    // TODO: Uncomment the code below to pass the first stage
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        io::stdin().read_line(&mut command)?;
        println!("{}: command not found", command.trim());
    }
    Ok(())
}