use super::{sa::Position, symbol::SymbolId};

pub type Result<T> = std::result::Result<T, SARError>;

#[derive(thiserror::Error, Debug)]
pub enum SARError {
    #[error("Invalid file format")]
    InvalidFileHeader,
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error("Symbol not found for id: {0}")]
    SymbolNotFound(SymbolId),
    #[error(transparent)]
    ImageError(#[from] image::ImageError),
    #[error("Projection error: positions are not normalized: {0:?}")]
    ProjectionError([Position; 4]),
}
