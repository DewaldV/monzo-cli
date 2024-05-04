use chrono::{Duration, Utc};
use monzo::{Client, Pot, Transaction};
use tokio::time::sleep;

use crate::currency::Amount;
use crate::{accounts, transactions};
use crate::{Error, Result};

fn print_pot_balance_row(account_type: &str, account_no: &str, pot_name: &str, balance: &str) {
    println!(
        "{:<14}   {:<14}   {:<30}   {:>12}",
        account_type, account_no, pot_name, balance
    );
}

pub async fn list(token: &str) -> Result<()> {
    let client = Client::new(token);

    let supported_accounts = accounts::get_supported_accounts(token).await?;

    print_pot_balance_row("Account Type", "Account Number", "Pot Name", "Balance");
    println!("-------------------------------------------------------------------------------");
    for (account_type, account) in supported_accounts {
        let pots = client.pots(&account.id).await?;

        for pot in pots.iter().filter(|pot| !pot.deleted) {
            let balance_value = Amount::from(pot.balance);
            print_pot_balance_row(
                &account_type.value(),
                &account.account_number,
                &pot.name,
                &balance_value.to_string(),
            );
        }
    }

    Ok(())
}

pub async fn deposit(
    token: &str,
    pot_name: &str,
    amount: &Amount,
    description: Option<String>,
) -> Result<()> {
    let client = Client::new(token);

    let found_pot = find_pot(token, pot_name).await?;
    let pot = found_pot.ok_or(Error::PotNotFound {
        pot_name: String::from(pot_name),
    })?;

    println!(
        "Found pot. Name: {}, Balance: {}",
        pot.name,
        Amount::from(pot.balance)
    );

    let amount_i: u32 = amount.pence.try_into()?;
    client
        .deposit_into_pot(&pot.id, &pot.current_account_id, amount_i)
        .await?;
    println!("Completed deposit. Name: {}, Amount: {}", pot.name, amount);

    if let Some(description) = description {
        let retry_max = 4;
        let retry_wait = tokio::time::Duration::from_millis(200);
        let mut attempt = 1;

        let pot_tx = loop {
            match find_deposit(token, &pot, amount).await {
                Ok(pot_tx) => {
                    break pot_tx;
                }
                Err(e) => {
                    if attempt >= retry_max {
                        return Err(e);
                    }
                    attempt += 1;
                    sleep(retry_wait).await;
                }
            }
        };

        transactions::annotate(token, &pot_tx.id, description).await?;
    }

    Ok(())
}

async fn find_deposit(token: &str, pot: &Pot, amount: &Amount) -> Result<Transaction> {
    let client = Client::new(token);
    let since = Utc::now() - Duration::minutes(1);
    let limit = 100;

    let transactions = client
        .transactions(&pot.current_account_id)
        .since(since)
        .limit(limit)
        .send()
        .await?;

    // Look for the pot deposit in the transaction list:
    // - It should be to the target pot
    // - It should have no existing notes set
    // - It should have a matching amount
    transactions
        .iter()
        .find(|tx| {
            tx.metadata
                .get("pot_id")
                .is_some_and(|pot_id| pot_id == &pot.id)
                && tx.metadata.get("notes").is_none()
                && tx.amount.abs() == amount.pence
        })
        .ok_or(Error::DepositTransactionNotFound)
        .cloned()
}

async fn find_pot(token: &str, name: &str) -> Result<Option<monzo::Pot>> {
    let client = Client::new(token);

    let supported_accounts = accounts::get_supported_accounts(token).await?;

    for (_, account) in supported_accounts {
        let pots = client.pots(&account.id).await?;
        let found_pot = pots
            .iter()
            .find(|pot| !pot.deleted && pot.name.to_lowercase() == name.to_lowercase());
        if found_pot.is_some() {
            return Ok(found_pot.cloned());
        }
    }

    Ok(None)
}
