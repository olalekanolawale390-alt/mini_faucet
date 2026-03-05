use alloy::{primitives::{Address}, providers::ProviderBuilder, signers::local::PrivateKeySigner, sol };
use std::{env};
use dotenv::dotenv;

sol! { 
    #[sol(rpc)] 
    contract MiniFaucet { 
         function deposit () public payable;
          function  claim(address _to) public payable;
    } 
} 

#[tokio::main]
pub async fn claim(address: String) -> String{
    dotenv().ok();

    let private_key = env::var("PRIVATE_KEY").unwrap();
    let signer: PrivateKeySigner =
        private_key.parse().expect("cannot parse key......");
 
    let provider = ProviderBuilder::new()
        .wallet(signer)
        .connect(&env::var("rpc_url").unwrap()).await.expect("cannotv connect");
    let recipient = address.parse::<Address>();
    if recipient.is_err(){
        if let Err(error) = recipient{return error.to_string();}
    }
    let recipient = recipient.unwrap();
    let contract = env::var("CONTRACT_ADDRESS").unwrap();
    let claim = contract.parse::<Address>().unwrap();
    let claim_instance = MiniFaucet::new(claim, provider);
    let claimer = claim_instance.claim(recipient);
    let req =  claimer.send()
    .await;
    if req.is_err() {
        if let Err(error) = req {return error.to_string();};
    }
    let receipt = req.unwrap().watch().await;
    receipt.unwrap().to_string()
}

