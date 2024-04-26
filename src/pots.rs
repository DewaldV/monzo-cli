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
            let balance_value = Amount::try_from(pot.balance)?;
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

pub async fn deposit(token: &str, pot_name: &str, amount: &Amount) -> Result<()> {
    let client = Client::new(token);

    let found_pot = find_pot(token, pot_name).await?;

    match found_pot {
        Some(pot) => {
            let balance = Amount::try_from(pot.balance)?;
            println!("Found pot. Name: {}, Balance: {}", pot.name, balance,);
            client
                .deposit_into_pot(&pot.id, &pot.current_account_id, amount.pence)
                .await?;
            println!("Completed deposit. Name: {}, Amount: {}", pot.name, amount);
        }
        None => {
            println!("No pot found with name: {}", pot_name);
        }
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
