use std::path::Path;

use crate::{accounts, currency::Amount, error::Result};

#[derive(Debug, serde::Deserialize)]
struct Row {
    account: accounts::AccountType,
    category: String,
    description: String,
    amount: Amount,
}

pub async fn run(_token: String, file: String) -> Result<()> {
    let path = Path::new(&file);
    println!("batch file path: {}", path.display());

    let mut reader = csv::Reader::from_path(path)?;
    for result in reader.deserialize() {
        let row: Row = result?;
        dbg!(row);
    }
    Ok(())
}
