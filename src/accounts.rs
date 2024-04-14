use monzo::Client;

use crate::currency;

#[derive(PartialEq, clap::ValueEnum, Clone, Copy)]
pub enum AccountType {
    Personal,
    Joint,
}

impl AccountType {
    pub fn value(self) -> String {
        match self {
            AccountType::Personal => String::from("Personal"),
            AccountType::Joint => String::from("Joint"),
        }
    }
}

impl TryFrom<&monzo::accounts::Type> for AccountType {
    type Error = String;

    fn try_from(value: &monzo::accounts::Type) -> Result<Self, Self::Error> {
        match value {
            monzo::accounts::Type::UkRetail => Ok(AccountType::Personal),
            monzo::accounts::Type::UkRetailJoint => Ok(AccountType::Joint),
            _ => Err(String::from("Unsupported account type")),
        }
    }
}

impl Into<monzo::accounts::Type> for AccountType {
    fn into(self) -> monzo::accounts::Type {
        match self {
            AccountType::Personal => monzo::accounts::Type::UkRetail,
            AccountType::Joint => monzo::accounts::Type::UkRetailJoint,
        }
    }
}

fn print_balance_row(account_type: &str, account_no: &str, created: &str, balance: &str) {
    println!(
        "{:<14}   {:<14}   {:<12}   {:>12}",
        account_type, account_no, created, balance
    );
}

pub async fn get_supported_accounts(
    token: &str,
) -> monzo::Result<Vec<(AccountType, monzo::Account)>> {
    let client = Client::new(token);

    let accounts = client.accounts().await?;

    let supported_accounts = accounts
        .iter()
        .filter_map(|acc| match AccountType::try_from(&acc.account_type) {
            Ok(acc_type) => Some((acc_type, acc.clone())),
            Err(_) => None,
        })
        .filter(|acc| !acc.1.account_number.is_empty())
        .collect();

    Ok(supported_accounts)
}

pub async fn list(token: &str) -> monzo::Result<()> {
    let client = Client::new(token);
    let supported_accounts = get_supported_accounts(token).await?;

    print_balance_row("Account Type", "Account No", "Created", "Balance");
    println!("-------------------------------------------------------------");

    for (account_type, account) in supported_accounts {
        let created = account.created.format("%Y-%m-%d").to_string();
        let balance = client.balance(&account.id).await?;
        let balance_value = currency::format_currency(balance.balance);
        print_balance_row(
            &account_type.value(),
            &account.account_number,
            &created,
            &balance_value,
        );
    }

    Ok(())
}
