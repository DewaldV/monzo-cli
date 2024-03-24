use monzo::Client;
use std::env;
use clap::{Parser, Subcommand};

#[tokio::main]
async fn main() -> monzo::Result<()> {
    let args = CLI::parse();

    match args.command {
        Commands::Balance => {
            balance().await?;
        }
    }
    Ok(())
}

async fn balance() -> monzo::Result<()> {
    let token = env::var("MONZO_ACCESS_TOKEN").expect("$MONZO_ACCESS_TOKEN is not set");

    // let client = Client::new(token).with_url("http://foo.bar.nope.not-a-thing");
    let client = Client::new(token);

    let accounts = client.accounts().await?;
    dbg!(&accounts);

    let account_id = &accounts[1].id;

    let pots = client.pots(account_id).await?;
    dbg!(&pots);

    let balance = client.balance(account_id).await?;
    dbg!(&balance);

    Ok(())
}

#[derive(Parser)]
#[command(name = "monzo")]
#[command(about = "A CLI for Monzo Finops", long_about = None)]
struct CLI {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Balance
}
