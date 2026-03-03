use crate::errors::MyShellError;

pub fn cat_file(fpath: String) -> Result<String, MyShellError> {

    let content = std::fs::read(fpath)?;    
    let ous = String::from_utf8_lossy(&content).trim().to_string();
    Ok(ous)
}