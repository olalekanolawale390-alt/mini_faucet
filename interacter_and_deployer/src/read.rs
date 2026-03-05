use alloy::{primitives::{U256, address}, providers::ProviderBuilder, sol};
use std::error::Error;
use::std::env;
use chrono::{DateTime, Utc};
 

sol! { 
    #[derive(Debug)]
    #[sol(rpc)] 
    contract MiniFaucet { 
        function faucetBalance() public view returns (uint256);
        function getBalance(address recipient) public view returns(uint256);
        function nextClaimTime() public view returns (uint256);
    } 
}
#[tokio::main]
pub async fn get_faucet_balance() -> Result<f64, Box<dyn Error>> {
    let provider = ProviderBuilder::new().connect(&env::var("rpc_url").unwrap()).await?;
    let faucet_balance = address!("0xeB3B6a3f084E237D364532dff4918589142eAbf8");
    let faucet_balance_instance = MiniFaucet::new(faucet_balance, provider);
    let get_faucet_balance = faucet_balance_instance.faucetBalance().call().await?;
    let get_faucet_balance = U256::from(get_faucet_balance).to::<u128>();
    let get_faucet_balance = get_faucet_balance as f64 / 1e18;
    println!("{:.4}", get_faucet_balance);
    Ok(get_faucet_balance)
}

#[tokio::main]
pub async fn get_wallet_balance() -> Result<f64, Box<dyn Error>> {
    let recipient = address!("0x5EE15251C47e60769F2E63605d4323ba54c07983");
    let provider = ProviderBuilder::new().connect(&env::var("rpc_url").unwrap()).await?;
    let wallet_balance = address!("0xeB3B6a3f084E237D364532dff4918589142eAbf8");
    let wallet_balance_instance = MiniFaucet::new(wallet_balance, provider);
    let get_wallet_balance = wallet_balance_instance.getBalance(recipient).call().await?;
    let get_wallet_balance = U256::from(get_wallet_balance).to::<u128>();
    let get_wallet_balance = get_wallet_balance as f64 / 1e18;

    println!("{:.4}", get_wallet_balance);
    Ok(get_wallet_balance)
}

#[tokio::main]
pub async fn next_claim() -> Result<String, Box<dyn Error>> {
    let recipient = address!("0x5EE15251C47e60769F2E63605d4323ba54c07983");
    let provider  = ProviderBuilder::new().connect(&env::var("rpc_url").unwrap()).await?;
    let next_claim = address!("0xeB3B6a3f084E237D364532dff4918589142eAbf8");
    let next_claim_instance = MiniFaucet::new(next_claim, provider);
    let req = next_claim_instance.nextClaimTime().from(recipient).call().await ;

    let get_next_claim = || {
        if let  Ok(result) = req {
        Ok(format!("{}", result))
    } else {
        return Err(req.err());
    }};
    let get_next_claim = get_next_claim().unwrap().parse::<i64>().unwrap();
    let dt: DateTime<Utc> = DateTime::from_timestamp_secs(get_next_claim).unwrap();
    let dt = dt.to_rfc2822();
    println!("{}", dt);
    Ok(dt)
}