use super::symbol::SymbolId;

pub type Result<T> = std::result::Result<T, SARError>;

#[derive(thiserror::Error, Debug)]
pub enum SARError {
    #[error("invalid file format")]
    InvalidFileHeader,
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error("symbol not found for id: {0}")]
    SymbolNotFound(SymbolId),
    #[error(transparent)]
    ImageError(#[from] image::ImageError),
    #[error("failed to create projection for points: from {0:?} to {1:?}")]
    ProjectionError([(f32, f32); 4], [(f32, f32); 4]),
}
