use monzo::Client;

use crate::currency;

fn print_balance_row(account_type: &str, account_no: &str, created: &str, balance: &str) {
    println!(
        "{:<14}   {:<14}   {:<12}   {:>12}",
        account_type, account_no, created, balance
    );
}

pub fn map_account_type(account_type: &monzo::accounts::Type) -> &str {
    match account_type {
        monzo::accounts::Type::UkRetail => "Personal",
        monzo::accounts::Type::UkRetailJoint => "Joint",
        _ => "Other",
    }
}

pub async fn balance(token: &str) -> monzo::Result<()> {
    let client = Client::new(token);

    let accounts = client.accounts().await?;
    let accounts_with_balances = accounts.iter().filter(|a| !a.account_number.is_empty());

    print_balance_row("Account Type", "Account No", "Created", "Balance");
    println!("-------------------------------------------------------------");

    for account in accounts_with_balances {
        let account_type = map_account_type(&account.account_type);
        let created = account.created.format("%Y-%m-%d").to_string();
        let balance = client.balance(&account.id).await?;
        let balance_value = currency::format_currency(balance.balance);
        print_balance_row(
            account_type,
            &account.account_number,
            &created,
            &balance_value,
        );
    }

    Ok(())
}
