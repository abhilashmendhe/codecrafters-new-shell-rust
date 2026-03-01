use crate::errors::MyShellError;


pub fn go_to_path(p_path: &str) -> Result<String, MyShellError> {


    let mut s = String::new();

    // Check 0: If immediate p_path is equal to ".."
    if p_path.eq("..") {
        std::env::set_current_dir(p_path)?;
        return Ok(s);
    }

    let p_path = p_path.trim();

    // read home path var
    let home_path = std::env::var("HOME")?;
    
    // Check 1: If p_path is empty string OR equal to `~`
    if p_path.len() == 0 || p_path.eq("~") {
        std::env::set_current_dir(&home_path)?;
        return Ok(s);
    }

    // Check 2: If p_path contains any space in between
    if let Some(_) = p_path.find(' ') {
        return Ok(format!("bash: cd: too many arguments"));
    }

    // Check 3: Handle errors for paths
    match std::env::set_current_dir(&p_path) {
        Ok(_) => { return Ok(s); }

        Err(_) => {
            // println!("{:?}", err.into_inner());
            s = format!("bash: cd: {}: No such file or directory", p_path);
            return Ok(s);
        },
    }
}