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
    cmd: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Balance,
    Pots {
        #[command(subcommand)]
        pot_cmd: PotsCommands,
    },
}

#[derive(Subcommand)]
enum PotsCommands {
    Balances {
        name: Option<String>
    },
}

#[tokio::main]
async fn main() -> monzo::Result<()> {
    let args = CLI::parse();

    match args.cmd {
        Commands::Balance => {
            balance(args.monzo_access_token).await?;
        }
        Commands::Pots{ pot_cmd } => {
            match pot_cmd {
                PotsCommands::Balances { name: _ } => {
                    pots(args.monzo_access_token).await?;
                }
            }
        }
    }
    Ok(())
}

fn print_balance_row(account_type: &str, account_no: &str, created: &str, balance: &str) {
    println!("{:<14}   {:<14}   {:<12}   {:>12}", account_type, account_no, created, balance);
}

fn map_account_type(account_type: &monzo::accounts::Type) -> &str {
   match account_type {
        monzo::accounts::Type::UkRetail => "Personal",
        monzo::accounts::Type::UkRetailJoint => "Joint",
        _ => "Other"
    }
}

async fn balance(token: impl Into<String>) -> monzo::Result<()> {
    let client = Client::new(token);

    let accounts = client.accounts().await?;
    let accounts_with_balances: Vec<&monzo::Account> = accounts.iter().filter(|a| !a.account_number.is_empty()).collect();

    print_balance_row("Account Type", "Account No", "Created", "Balance");
    println!("-------------------------------------------------------------");

    for account in accounts_with_balances.iter() {
        let account_type = map_account_type(&account.account_type);
        let created = account.created.format("%Y-%m-%d").to_string();
        let balance = client.balance(&account.id).await?;
        let balance_value = currency::format_currency(balance.balance);
        print_balance_row(account_type, &account.account_number, &created, &balance_value);
    }

    Ok(())
}

fn print_pot_balance_row(account_type: &str, account_no: &str, pot_name: &str, balance: &str) {
    println!("{:<14}   {:<14}   {:<30}   {:>12}", account_type, account_no, pot_name, balance);
}

async fn pots(token: impl Into<String>) -> monzo::Result<()> {
    let client = Client::new(token);

    let accounts = client.accounts().await?;
    let accounts_with_pots: Vec<&monzo::Account> = accounts.iter().filter(|a| !a.account_number.is_empty()).collect();

    print_pot_balance_row("Account Type", "Account Number", "Pot Name", "Balance");
    println!("-------------------------------------------------------------------------------");
    for account in accounts_with_pots.iter() {
        let pots = client.pots(&account.id).await?;
        let active_pots: Vec<&monzo::Pot> = pots.iter().filter(|p| !p.deleted).collect();
        let account_type = map_account_type(&account.account_type);
        for pot in active_pots.iter() {
            let balance_value = currency::format_currency(pot.balance);
            print_pot_balance_row(account_type, &account.account_number, &pot.name, &balance_value);
        }
    }

    Ok(())
}
