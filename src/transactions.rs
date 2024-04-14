use std::collections::HashMap;

use chrono::{prelude::*, Duration};
use monzo::Client;

use crate::accounts;
use crate::currency;

pub async fn list(token: &str, account_type: accounts::AccountType) -> monzo::Result<()> {
    let client = Client::new(token);

    let accounts = client.accounts().await?;
    let found_account = accounts
        .iter()
        .find(|acc| acc.account_type == account_type.into());

    match found_account {
        Some(acc) => {
            println!("Transactions for account: {}", account_type.value());
            println!("");
            print_transaction_row("Created", "Category", "Description", "Amount");
            println!("-----------------------------------------------------------------------------------------------------------");

            let transactions = client
                .transactions(&acc.id)
                .since(Utc::now() - Duration::days(5))
                .limit(10)
                .send()
                .await?;

            for tx in transactions.iter() {
                let created = &tx.created.format("%Y-%m-%d").to_string();
                let amount = &currency::format_currency(tx.amount);
                print_transaction_row(created, &tx.category, &tx.id, amount);
            }
        }
        None => {
            println!("No account found for type: {}", account_type.value());
        }
    }

    Ok(())
}

pub async fn annotate(token: &str, transaction_id: &str, note: String) -> monzo::Result<()> {
    let client = Client::new(token);

    let metadata = HashMap::from([(String::from("notes"), note)]);

    let tx = client
        .annotate_transaction(transaction_id, metadata)
        .await?;

    println!("Note added.");
    println!("");
    let created = &tx.created.format("%Y-%m-%d").to_string();
    let amount = &currency::format_currency(tx.amount);
    print_transaction_row("Created", "Category", "Note", "Amount");
    println!("-----------------------------------------------------------------------------------------------------------");
    print_transaction_row(created, &tx.category, &tx.notes, amount);

    Ok(())
}

fn print_transaction_row(created: &str, category: &str, description: &str, amount: &str) {
    println!(
        "{:<12}   {:<14}   {:<60}   {:>12}",
        created, category, description, amount
    );
}
