pub type Result<T> = std::result::Result<T, SARError>;

#[derive(thiserror::Error, Debug)]
pub enum SARError {
    #[error("Invalid file format")]
    InvalidFileFormat,
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error("Symbol not found")]
    SymbolNotFound,
    #[error(transparent)]
    ImageError(#[from] image::ImageError),
    #[error("Projection error")]
    ProjectionError,
}
