use thiserror::Error;


#[derive(Error, Debug)]
pub enum MyShellError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Var error: {0}")]
    Var(#[from] std::env::VarError),

    #[error("Nothing after slash (\\): {0}")]
    NothingAfterSlash(String),
}