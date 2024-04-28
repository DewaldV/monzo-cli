use std::collections::HashMap;

use chrono::{prelude::*, Duration};
use monzo::Client;

use crate::accounts;
use crate::currency::Amount;
use crate::Result;

pub async fn list(
    token: &str,
    account_type: accounts::AccountType,
    since: Option<DateTime<Utc>>,
    limit: u16,
) -> Result<()> {
    let client = Client::new(token);

    let accounts = client.accounts().await?;
    let found_account = accounts
        .iter()
        .find(|acc| acc.account_type == account_type.into());

    match found_account {
        None => {
            println!("No account found for type: {}", account_type.value());
            return Ok(());
        }

        Some(acc) => {
            println!("Transactions for account: {}", account_type.value());
            println!("");
            print_transaction_row("Created", "Category", "Description", "Amount");
            println!("-----------------------------------------------------------------------------------------------------------");

            let since = since.unwrap_or(Utc::now() - Duration::days(1));

            let transactions = client
                .transactions(&acc.id)
                .since(since)
                .limit(limit)
                .send()
                .await?;

            for tx in transactions.iter() {
                let created = &tx.created.format("%Y-%m-%d").to_string();
                let amount = Amount::from(tx.amount);
                print_transaction_row(created, &tx.category, &tx.id, &amount.to_string());
            }
        }
    }

    Ok(())
}

pub async fn annotate(token: &str, transaction_id: &str, note: String) -> Result<()> {
    let client = Client::new(token);

    let metadata = HashMap::from([(String::from("notes"), note)]);

    let tx = client
        .annotate_transaction(transaction_id, metadata)
        .await?;

    println!("Note added.");
    println!("");
    let created = &tx.created.format("%Y-%m-%d").to_string();
    let amount = Amount::from(tx.amount);
    print_transaction_row("Created", "Category", "Note", "Amount");
    println!("-----------------------------------------------------------------------------------------------------------");
    print_transaction_row(created, &tx.category, &tx.notes, &amount.to_string());

    Ok(())
}

fn print_transaction_row(created: &str, category: &str, description: &str, amount: &str) {
    println!(
        "{:<12}   {:<14}   {:<60}   {:>12}",
        created, category, description, amount
    );
}
