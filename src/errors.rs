use thiserror::Error;


#[derive(Error, Debug)]
pub enum MyShellError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}