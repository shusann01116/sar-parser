pub type Result<T> = std::result::Result<T, SARError>;

#[derive(thiserror::Error, Debug)]
pub enum SARError {
    #[error("Invalid file format")]
    InvalidFileFormat,
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}
