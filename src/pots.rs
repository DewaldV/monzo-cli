use chrono::{Duration, Utc};
use monzo::Client;

use crate::accounts;
use crate::currency::Amount;
use crate::Result;

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

    if found_pot.is_none() {
        println!("No pot found with name: {}", pot_name);
        return Ok(());
    }

    let pot = found_pot.expect("none checked above so this is safe");

    let balance = Amount::from(pot.balance);
    println!("Found pot. Name: {}, Balance: {}", pot.name, balance);

    let amount_i: u32 = amount.pence.try_into()?;
    client
        .deposit_into_pot(&pot.id, &pot.current_account_id, amount_i)
        .await?;
    println!("Completed deposit. Name: {}, Amount: {}", pot.name, amount);

    if let Some(description) = description {
        let since = Utc::now() - Duration::minutes(5);
        let limit = 10;

        let transactions = client
            .transactions(&pot.current_account_id)
            .since(since)
            .limit(limit)
            .send()
            .await?;

        // transactions
        //     .iter()
        //     .filter(|tx| tx.metadata.contains_key("pot_id"))
        //     .find(|tx| tx.metadata.get(k));

        // metadata: {
        //     "ledger_committed_timestamp_earliest": "2024-04-26T21:17:37.867Z",
        //     "pot_deposit_id": "potdep_0000AhIBZeZKQxtcO5qKSv",
        //     "ledger_insertion_id": "entryset_0000AhIBZeQSxvHj6Dcsy1",
        //     "user_id": "user_00009HutzjJh6uIxmYfk6z",
        //     "external_id": "user_00009HutzjJh6uIxmYfk6z:VG5PeJCYVY",
        //     "pot_account_id": "acc_0000AgbkY0OhRmP0ooPx69",
        //     "trigger": "user",
        //     "ledger_committed_timestamp_latest": "2024-04-26T21:17:37.867Z",
        //     "pot_id": "pot_0000AgbkY00Euhtjk7z0td",
        // },
        // let tx_list = transactions::list(token, account_type, since, limit).await?;
        // transactions::annotate(token, transaction_id, description).await?
    }

    Ok(())
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
