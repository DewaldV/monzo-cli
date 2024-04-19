#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Monzo API error: {0}")]
    MonzoAPI(#[from] monzo::Error),

    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
