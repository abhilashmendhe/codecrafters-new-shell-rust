use crate::errors::MyShellError;

pub fn cat_file(fpath: String) -> Result<String, MyShellError> {

    if fpath.is_empty() {
        return Ok("".to_string());
    }
    // let fpath = format!("\'{}\'", fpath);
    // println!("START - /src/quoting/cat_file.rs : fpath - {}",fpath);
    let content = std::fs::read(fpath)?;    
    let ous = String::from_utf8_lossy(&content).trim().to_string();
    // println!("END - /src/quoting/cat_file.rs : {}\n", ous);
    Ok(ous)
}