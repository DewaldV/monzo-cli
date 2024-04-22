pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unsupported account type {account_type:?}")]
    UnsupportedAccountType { account_type: monzo::accounts::Type },

    // External
    #[error("Monzo API error: {0}")]
    MonzoAPI(#[from] monzo::Error),

    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
}
