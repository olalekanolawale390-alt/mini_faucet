#![allow(unused)]
use alloy::{primitives::{address, utils::{format_ether, Unit}, U256},
    providers::ProviderBuilder,
    signers::local::PrivateKeySigner,
    sol,
};
use dotenv::dotenv;
use::std::env;
use std::error::Error;

sol!(
    #[sol(rpc)]
    MiniFaucet,
    "src/artifacts/MiniFaucet.json"
);
 
#[tokio::main]
pub async fn deploy() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let rpc = &env::var("rpc_url").unwrap();
    let private_key = std::env::var("PRIVATE_KEY").expect("....key not availale......");
    let wallet:PrivateKeySigner = private_key.parse().expect("");
    let provider = ProviderBuilder::new().wallet(wallet).connect(rpc).await?;
    let deployer = MiniFaucet::deploy(&provider).await?;
 
    println!("Deployer deployed at: {}", deployer.address());

    Ok(())
}