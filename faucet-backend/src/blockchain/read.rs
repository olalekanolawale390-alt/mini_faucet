use alloy::{primitives::{Address, U256, address}, providers::ProviderBuilder, sol};
use std::env;
use dotenv::dotenv;
use chrono::{DateTime, Utc};
 

sol! { 
    #[derive(Debug)]
    #[sol(rpc)] 
    contract MiniFaucet { 
        function faucetBalance() public view returns (uint256);
        function getBalance(address recipient) public view returns(uint256);
        function nextClaimTime(address addy) public view returns (uint256);
    } 
}
#[tokio::main]
pub async fn get_faucet_balance() -> String{
    dotenv().ok();
    let provider = ProviderBuilder::new().connect(&env::var("rpc_url").unwrap()).await;
    let faucet_balance = address!("0x3AC5a5f60753bbfaD93B668A0bEC5c8fA0E647be");
    let faucet_balance_instance = MiniFaucet::new(faucet_balance, provider.unwrap());
    let get_faucet_balance = faucet_balance_instance.faucetBalance().call().await;
  
    match get_faucet_balance {
        Ok(result) => {
            let faucet_balance = U256::from(result).to::<u128>();
            let faucet_balance = faucet_balance as f64 / 1e18;
            faucet_balance.to_string()
        }
        Err(e) =>  e.to_string()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Resulter<T> (T);
impl Resulter<String>{
    pub fn display(&self) -> String{
        format!("{}", self.0)
    }
}
#[tokio::main]
pub async fn get_wallet_balance(address: String) -> Resulter<String> {
    dotenv().ok();
    let recipient = address.parse::<Address>();
    if recipient.is_err() {
        if let Err(e) = recipient {return Resulter(e.to_string())}
    }
    let recipient = recipient.unwrap();
    let provider = ProviderBuilder::new().connect(&env::var("rpc_url").unwrap()).await;
    let wallet_balance = address!("0x3AC5a5f60753bbfaD93B668A0bEC5c8fA0E647be");
    let wallet_balance_instance = MiniFaucet::new(wallet_balance, provider.unwrap());
    let get_wallet_balance = wallet_balance_instance.getBalance(recipient).call().await;
    match get_wallet_balance {
        result => {
            let wallet_balance = U256::from(result.unwrap()).to::<u128>();
            let wallet_balance = wallet_balance as f64 / 1e18;
            Resulter(wallet_balance.to_string())
        }
    }
    // get_wallet_balance
}

#[tokio::main]
pub async fn next_claim(address: String) -> String {
    dotenv().ok();
    let user = address.parse::<Address>();
    if let Err(error) = user {return error.to_string()}
    let user = user.unwrap();
    let provider  = ProviderBuilder::new().connect(&env::var("rpc_url").unwrap()).await.unwrap();
    let next_claim = address!("0x3AC5a5f60753bbfaD93B668A0bEC5c8fA0E647be");
    let next_claim_instance = MiniFaucet::new(next_claim, provider);
    let req = next_claim_instance.nextClaimTime(user).call().await ;

    if let Err(error) = req {return error.to_string();}
    let get_next_claim = req.unwrap().to_string();
    let get_next_claim = get_next_claim.parse::<i64>().unwrap();
    let dt: DateTime<Utc> = DateTime::from_timestamp_secs(get_next_claim).unwrap();
    let dt = dt.to_rfc2822();
    dt
}