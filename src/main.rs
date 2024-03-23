use monzo::Client;
use std::env;

#[tokio::main]
async fn main() -> monzo::Result<()> {
    let token = env::var("MONZO_ACCESS_TOKEN").expect("$MONZO_ACCESS_TOKEN is not set");

    // You can create a simple monzo client using only an access token
    let quick_client = Client::new(token);

    // get a list of accounts
    let accounts = quick_client.accounts().await?;
    dbg!(&accounts);

    // get the id of one of the accounts
    let account_id = &accounts[1].id;

    // get the balance of that account
    let balance = quick_client.balance(account_id).await?;
    dbg!(&balance);

    Ok(())
}
