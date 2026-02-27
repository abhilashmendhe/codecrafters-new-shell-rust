use std::{env, os::unix::fs::PermissionsExt};

use crate::errors::MyShellError;

pub fn read_type_path(c_type: &str) -> Result<String, MyShellError> {

    let type_path = env::var("PATH")?;
    let mut output = format!("{}: not found", c_type);
    for p in type_path.split(":") {
        // println!("{}", p);
        for entry_result in  std::fs::read_dir(p)? {

            let entry = entry_result?;
            
            // println!("{:?}", entry.path().display());
            let f_meta = entry.metadata()?;
            let perm_mode = f_meta.permissions().mode();
            // println!("{:?}",f_meta);
            if entry.file_name().eq(c_type) && perm_mode & 0o111 != 0 {
                output = format!("{} is {}/{}", c_type, p, c_type);
                return Ok(output);
                // break;
            }
        }
    }
    Ok(output)
}