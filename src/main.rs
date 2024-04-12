use clap::{Parser, Subcommand};

mod accounts;
mod currency;
mod pots;
mod transactions;

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
    Accounts {
        #[command(subcommand)]
        acc_cmd: AccountCommands,
    },
    Pots {
        #[command(subcommand)]
        pot_cmd: PotsCommands,
    },
    Transactions {
        #[command(subcommand)]
        tx_cmd: TransactionCommands,
    },
}

#[derive(Subcommand)]
enum AccountCommands {
    List,
}

#[derive(Subcommand)]
enum PotsCommands {
    List { name: Option<String> },
    Deposit { name: String, value: String },
}

#[derive(Subcommand)]
enum TransactionCommands {
    List { account_type: accounts::Type },
    Annotate { transaction_id: String },
}

#[tokio::main]
async fn main() -> monzo::Result<()> {
    let args = CLI::parse();

    match args.cmd {
        Commands::Accounts { acc_cmd } => match acc_cmd {
            AccountCommands::List => {
                accounts::list(&args.monzo_access_token).await?;
            }
        },
        Commands::Pots { pot_cmd } => match pot_cmd {
            PotsCommands::List { name: _ } => {
                pots::list(&args.monzo_access_token).await?;
            }
            PotsCommands::Deposit { name, value } => {
                pots::deposit(&args.monzo_access_token, &name, &value).await?;
            }
        },
        Commands::Transactions { tx_cmd } => match tx_cmd {
            TransactionCommands::List { account_type } => {
                transactions::list(&args.monzo_access_token, account_type).await?;
            }
            TransactionCommands::Annotate { transaction_id } => {
                transactions::annotate(&args.monzo_access_token, &transaction_id).await?;
            }
        },
    }
    Ok(())
}
