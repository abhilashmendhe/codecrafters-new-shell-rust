use std::{env, os::unix::fs::PermissionsExt, path::PathBuf};

use crate::{errors::MyShellError, intro::check_type::ShellFileType};

pub fn check_executable(c_type: &str) -> Result<ShellFileType, MyShellError> {

    let type_path = env::var("PATH")?;

    let mut shell_f_type = ShellFileType {
        f_name: c_type.to_string(),
        f_path: PathBuf::new(),
        // output_format: format!("{}: not found", c_type),
        is_exe: false,
    };

    for p in type_path.split(":") {
        // println!("{}", p);
        for entry_result in  std::fs::read_dir(p)? {

            let entry = entry_result?;
            
            // println!("{:?}", entry.path().display());
            let f_meta = entry.metadata()?;
            let perm_mode = f_meta.permissions().mode();
            // println!("{:?}",f_meta);
            if entry.file_name().eq(c_type) && perm_mode & 0o111 != 0 {
                shell_f_type.f_path = entry.path();
                shell_f_type.is_exe = true;
                return Ok(shell_f_type);
                // break;
            }
        }
    }
    Ok(shell_f_type)
}