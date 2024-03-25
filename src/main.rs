use monzo::Client;
use clap::{Parser, Subcommand};

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

async fn balance(token: impl Into<String>) -> monzo::Result<()> {
    let client = Client::new(token);

    let accounts = client.accounts().await?;

    let accounts_with_balances: Vec<&monzo::Account> = accounts.iter().filter(|a| a.account_number.is_empty()).collect();

    for account in accounts_with_balances.iter() {
        let balance = client.balance(&account.id).await?;
        dbg!(&account.account_type);
        dbg!(&balance);
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
