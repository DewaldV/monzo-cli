use monzo::Client;
use std::env;

#[tokio::main]
async fn main() -> monzo::Result<()> {
    let token = env::var("MONZO_ACCESS_TOKEN").expect("$MONZO_ACCESS_TOKEN is not set");

    let client = Client::new(token).with_url("http://foo.bar.nope.not-a-thing");

    let accounts = client.accounts().await?;
    dbg!(&accounts);

    let account_id = &accounts[1].id;

    let balance = client.balance(account_id).await?;
    dbg!(&balance);

    Ok(())
}
