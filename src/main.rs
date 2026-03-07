use codecrafters_shell::{errors::MyShellError, start_run};

fn main() -> Result<(), MyShellError> {
    let _ = start_run()?;   
    Ok(())
}
