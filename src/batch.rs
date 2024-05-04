use std::{fmt::Display, path::Path};

use crate::{
    accounts,
    config::{self, Config},
    currency::Amount,
    error::Result,
    pots, Error,
};

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

struct CSVMetrics {
    rows: u16,
    success: u16,
    failed: u16,
}

impl CSVMetrics {
    fn new() -> Self {
        CSVMetrics {
            rows: 0,
            success: 0,
            failed: 0,
        }
    }

    fn success(&mut self) {
        self.rows += 1;
        self.success += 1;
    }

    fn failure(&mut self) {
        self.rows += 1;
        self.failed += 1;
    }
}

impl Display for CSVMetrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "total_rows={}, successful={}, failed={}",
            self.rows, self.success, self.failed
        )
    }
}

pub async fn run(token: String, batch_file: String, config_file: String) -> Result<()> {
    let config = config::load(&config_file)?;
    println!("loaded config from {}", config_file);

    let batch_path = Path::new(&batch_file);
    println!("starting batch run for {}", batch_file);

    let mut metrics = CSVMetrics::new();

    let mut reader = csv::Reader::from_path(batch_path)?;
    for result in reader.deserialize() {
        let row: Row = result?;

        match run_row(row, &token, &config).await {
            Ok(_v) => {
                metrics.success();
                println!("processed row successfully")
            }
            Err(e) => {
                metrics.failure();
                println!("error processing row, {}", e)
            }
        }
    }

    println!("completed batch run.");
    println!("{}", metrics);
    Ok(())
}

async fn run_row(row: Row, token: &String, config: &Config) -> Result<()> {
    println!("executing batch row, {}", row);
    let destination = config
        .get_pot_for_deposit(&row.account)
        .ok_or(Error::PotNotFound {
            pot_name: row.account.value(),
        })?;

    pots::deposit(
        &token,
        &destination.pot_name,
        &row.amount,
        Some(row.description),
    )
    .await
}
