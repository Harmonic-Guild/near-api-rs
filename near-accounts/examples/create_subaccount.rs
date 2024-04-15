//use near_providers::Provider;
use near_accounts::Account;
use near_crypto::InMemorySigner;
use near_primitives::types::Balance;
use near_providers::JsonRpcProvider;
use std::sync::Arc;
mod utils;
use near_primitives::types::AccountId;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let signer_account_id: AccountId = utils::input("Enter the signer Account ID: ")?.parse()?;
    let signer_secret_key = utils::input("Enter the signer's private key: ")?.parse()?;
    let new_account_id: AccountId =
        utils::input("Enter the account name of new account ")?.parse()?;
    let signer = InMemorySigner::from_secret_key(signer_account_id.clone(), signer_secret_key);

    // Amount to transfer to the new account
    let amount: Balance = 10_000_000_000_000_000_000_000; // Example amount in yoctoNEAR

    let new_key_pair = near_crypto::SecretKey::from_random(near_crypto::KeyType::ED25519);
    let provider = Arc::new(JsonRpcProvider::new("https://rpc.testnet.near.org"));
    let signer = Arc::new(signer);

    let account = Account::new(signer_account_id, signer, provider);
    // Call create_account
    let result = account
        .create_account(&new_account_id, new_key_pair.public_key(), amount)
        .await;

    match result {
        Ok(res) => {
            println!("transaction: {:#?}", res.transaction);
            println!("status: {:#?}", res.status);
            println!("receipts_outcome {:#?}", res.transaction_outcome);
        }
        Err(err) => println!("Error: {:#?}", err),
    }

    println!("=============================================================");
    println!("New Account ID: {}", new_account_id);
    println!("    Secret Key: {}", new_key_pair);
    println!("    Public Key: {}", new_key_pair.public_key());
    println!("       Deposit: {}", amount);
    println!("-------------------------------------------------------------");

    Ok(())
}
