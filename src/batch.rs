use std::{fmt::Display, path::Path};

use crate::{accounts, currency::Amount, error::Result};

#[derive(Debug, serde::Deserialize)]
struct Row {
    account: accounts::AccountType,
    category: String,
    description: String,
    amount: Amount,
}

impl Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "account={}, category={}, description={}, amount={}",
            self.account.value(),
            self.category,
            self.description,
            self.amount
        )
    }
}

pub async fn run(token: String, file: String) -> Result<()> {
    let path = Path::new(&file);
    println!("starting batch run for file: {}", path.display());

    let mut reader = csv::Reader::from_path(path)?;
    for result in reader.deserialize() {
        let row: Row = result?;

        println!("executing batch row, {}", row);

        // let pot_name = config::pot_for(row.account, row.category);
        // let deposit = pots::deposit(&token, pot_name, &row.amount).await?;
        // let deposit_tx = transactions::find_transaction(&token, depo).await?;
        // transactions::annotate(&token, &deposit_tx.id, row.description).await?;
    }
    Ok(())
}
