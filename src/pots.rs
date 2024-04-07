use monzo::Client;

use crate::accounts;
use crate::currency;

fn print_pot_balance_row(account_type: &str, account_no: &str, pot_name: &str, balance: &str) {
    println!(
        "{:<14}   {:<14}   {:<30}   {:>12}",
        account_type, account_no, pot_name, balance
    );
}

pub async fn list(token: &str) -> monzo::Result<()> {
    let client = Client::new(token);

    let accounts = client.accounts().await?;
    let supported_accounts = accounts.iter().filter(|acc| !acc.account_number.is_empty());

    print_pot_balance_row("Account Type", "Account Number", "Pot Name", "Balance");
    println!("-------------------------------------------------------------------------------");
    for account in supported_accounts {
        let account_type = accounts::Type::try_from(&account.account_type)
            .expect("already filtered for account types that converted successfully");

        let pots = client.pots(&account.id).await?;

        for pot in pots.iter().filter(|p| !p.deleted) {
            let balance_value = currency::format_currency(pot.balance);
            print_pot_balance_row(
                &account_type.value(),
                &account.account_number,
                &pot.name,
                &balance_value,
            );
        }
    }

    Ok(())
}

pub async fn deposit(token: &str, pot_name: &str, amount: &str) -> monzo::Result<()> {
    let pence = currency::parse_currency(amount).expect("should be an amount ex: 1,203.05");

    let client = Client::new(token);

    let accounts = client.accounts().await?;
    let found_pot = find_pot(token, pot_name, &accounts).await?;

    match found_pot {
        Some(pot) => {
            println!(
                "Found pot. Name: {}, Balance: {}",
                pot.name,
                currency::format_currency(pot.balance)
            );
            client
                .deposit_into_pot(&pot.id, &pot.current_account_id, pence)
                .await?;
            println!("Completed deposit. Name: {}, Amount: {}", pot.name, amount);
        }
        None => {
            println!("No pot found with name: {}", pot_name);
        }
    }

    Ok(())
}

async fn find_pot(
    token: &str,
    name: &str,
    accounts: &Vec<monzo::Account>,
) -> monzo::Result<Option<monzo::Pot>> {
    let client = Client::new(token);

    for account in accounts.iter().filter(|acc| !acc.account_number.is_empty()) {
        let pots = client.pots(&account.id).await?;
        let found_pot = pots
            .iter()
            .find(|p| !p.deleted && p.name.to_lowercase() == name.to_lowercase());
        if found_pot.is_some() {
            return Ok(found_pot.cloned());
        }
    }

    Ok(None)
}
