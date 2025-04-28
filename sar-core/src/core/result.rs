use super::symbol::SymbolId;

/// A specialized Result type for SAR operations.
///
/// This is a type alias for `std::result::Result<T, SARError>`, providing
/// a consistent error handling type throughout the SAR codebase. It uses
/// `SARError` as the error type for all operations that can fail.
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
