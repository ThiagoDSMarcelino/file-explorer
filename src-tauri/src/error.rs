#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Generic(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}
