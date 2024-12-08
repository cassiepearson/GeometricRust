use thiserror::Error;

#[derive(Debug, Error)]
pub enum GeomError {
    #[error("Unsupported type found. Failed to {0}")]
    UnsupportedType(String),
}
