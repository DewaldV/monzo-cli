use chrono::{DateTime, Utc};
use clap::{Parser, Subcommand};
use currency::Amount;

mod accounts;
mod batch;
mod currency;
mod error;
mod pots;
mod transactions;

pub use self::error::{Error, Result};

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
    Batch {
        #[command(subcommand)]
        batch_cmd: BatchCommands,
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
enum BatchCommands {
    Run { file: String },
}

#[derive(Subcommand)]
enum PotsCommands {
    List {
        name: Option<String>,
    },
    Deposit {
        name: String,
        value: Amount,

        #[arg(short, long)]
        description: Option<String>,
    },
}

#[derive(Subcommand)]
enum TransactionCommands {
    List {
        account_type: accounts::AccountType,

        #[arg(short, long)]
        since: Option<DateTime<Utc>>,

        #[arg(short, long, default_value = "10")]
        limit: u16,
    },
    UpdateNote {
        transaction_id: String,
        note: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = CLI::parse();

    match args.cmd {
        Commands::Accounts { acc_cmd } => match acc_cmd {
            AccountCommands::List => {
                accounts::list(&args.monzo_access_token).await?;
            }
        },
        Commands::Batch { batch_cmd } => match batch_cmd {
            BatchCommands::Run { file } => {
                batch::run(args.monzo_access_token, file).await?;
            }
        },
        Commands::Pots { pot_cmd } => match pot_cmd {
            PotsCommands::List { name: _ } => {
                pots::list(&args.monzo_access_token).await?;
            }
            PotsCommands::Deposit {
                name,
                value,
                description,
            } => {
                pots::deposit(&args.monzo_access_token, &name, &value, description).await?;
            }
        },
        Commands::Transactions { tx_cmd } => match tx_cmd {
            TransactionCommands::List {
                account_type,
                since,
                limit,
            } => {
                transactions::list(&args.monzo_access_token, account_type, since, limit).await?;
            }
            TransactionCommands::UpdateNote {
                transaction_id,
                note,
            } => {
                transactions::annotate(&args.monzo_access_token, &transaction_id, note).await?;
            }
        },
    }
    Ok(())
}
