use clap::{Parser, Subcommand};

mod accounts;
mod currency;
mod pots;

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
    Balance {
        name: Option<String>
    },
    Deposit {
        name: String,
        value: String,
    },
}

#[tokio::main]
async fn main() -> monzo::Result<()> {
    let args = CLI::parse();

    match args.cmd {
        Commands::Balance => {
            accounts::balance(&args.monzo_access_token).await?;
        }
        Commands::Pots{ pot_cmd } => {
            match pot_cmd {
                PotsCommands::Balance { name: _ } => {
                    pots::balance(&args.monzo_access_token).await?;
                }
                PotsCommands::Deposit { name, value } => {
                    pots::deposit(&args.monzo_access_token, &name, &value).await?;
                }
            }
        }
    }
    Ok(())
}
