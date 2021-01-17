use web3::{transports, Web3};
use tokio;

#[tokio::main]
async fn main() -> web3::Result<()> {
    let transport = transports::Http::new("http://substrate:5000")?;
    let web3 = Web3::new(transport);

    println!("Calling accounts.");
    let mut accounts = web3.eth().accounts().await?;
    println!("Accounts: {:?}", accounts);
    accounts.push("00a329c0648769a73afac7f9381e08fb43dbea72".parse().unwrap());

    println!("Calling balance.");
    for account in accounts {
        let balance = web3.eth().balance(account, None).await?;
        println!("Balance of {:?}: {}", account, balance);
    }

    Ok(())
}