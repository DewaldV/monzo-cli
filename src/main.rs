use monzo::Client;
use clap::{Parser, Subcommand};

mod currency;

#[derive(Parser)]
#[command(name = "monzo")]
#[command(about = "A CLI for Monzo Finops", long_about = None)]
struct CLI {
    #[clap(long, env, hide_env_values(true))]
    monzo_access_token: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Balance,
    Pots,
}

#[tokio::main]
async fn main() -> monzo::Result<()> {
    let args = CLI::parse();

    match args.command {
        Commands::Balance => {
            balance(args.monzo_access_token).await?;
        }
        Commands::Pots => {
            pots(args.monzo_access_token).await?;
        }
    }
    Ok(())
}

fn print_balance_row(account_type: &str, created: &str, balance: &str) {
    println!("{:<14} | {:<12} | {:>12}", account_type, created, balance);
}

async fn balance(token: impl Into<String>) -> monzo::Result<()> {
    let client = Client::new(token);

    let accounts = client.accounts().await?;

    let accounts_with_balances: Vec<&monzo::Account> = accounts.iter().filter(|a| !a.account_number.is_empty()).collect();

    print_balance_row("Account Type", "Created", "Balance");
    println!("---------------|--------------|-------------");

    for account in accounts_with_balances.iter() {
        let account_type = match account.account_type {
            monzo::accounts::Type::UkRetail => "Personal",
            monzo::accounts::Type::UkRetailJoint => "Joint",
            _ => "Other"
        };

        let created = account.created.format("%Y-%m-%d").to_string();
        let balance = client.balance(&account.id).await?;
        let balance_value = currency::format_currency(balance.balance);
        print_balance_row(account_type, created.as_str(), balance_value.as_str());
    }

    Ok(())
}

async fn pots(token: impl Into<String>) -> monzo::Result<()> {
    let client = Client::new(token);

    let accounts = client.accounts().await?;

    for account in accounts.iter() {
        let pots = client.pots(&account.id).await?;
        let active_pots: Vec<&monzo::Pot> = pots.iter().filter(|p| !p.deleted).collect();

        dbg!(&active_pots);
    }

    Ok(())
}
