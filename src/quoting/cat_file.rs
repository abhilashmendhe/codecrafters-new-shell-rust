use crate::{errors::MyShellError, quoting::file_exists::file_exists_check};

pub fn cat_file(fpath: String) -> Result<String, MyShellError> {

    if fpath.is_empty() {
        return Ok("".to_string());
    }
    // let fpath = format!("\'{}\'", fpath);
    // println!("START - /src/quoting/cat_file.rs : fpath - {}",fpath); 

    let file_exists = file_exists_check(crate::CMD::CAT, &fpath)?;

    let ous = if file_exists.exists {      
        let raw_content = std::fs::read(&fpath)?;
        String::from_utf8_lossy(&raw_content).trim().to_string()
    } else {
        file_exists.message
    };

    // println!("END - /src/quoting/cat_file.rs : {}\n", ous);
    Ok(ous)
}