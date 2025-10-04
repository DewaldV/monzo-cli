pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unsupported account type {account_type:?}")]
    UnsupportedAccountType { account_type: monzo::accounts::Type },

    #[error("no pot found with name={pot_name}")]
    PotNotFound { pot_name: String },

    #[error("deposit transactions not found, unable to annotate")]
    DepositTransactionNotFound,

    #[error("CSV parsing error: {0}")]
    CSVParseError(String),

    // External
    #[error("Monzo API error: {0}")]
    MonzoAPI(#[from] monzo::Error),

    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),

    #[error("TryFromInt error: {0}")]
    TryFromInt(#[from] std::num::TryFromIntError),

    #[error("CSV error: {0}")]
    CSV(#[from] csv::Error),

    #[error("TOML error: {0}")]
    TOML(#[from] toml::de::Error),
}
