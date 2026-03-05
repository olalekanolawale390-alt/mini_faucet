use alloy::{primitives::{ address},
    providers::ProviderBuilder,
    signers::local::PrivateKeySigner,
    sol,
};
use std::error::Error;
use dotenv::dotenv;
use::std::env;

sol! { 
    #[sol(rpc)] 
    contract MiniFaucet { 
         function deposit () public payable;
          function  claim(address _to) public payable;
    } 
} 

#[tokio::main]
pub async fn claim() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let private_key = std::env::var("PRIVATE_KEY").expect("");
    let signer: PrivateKeySigner =
        private_key.parse().expect("cannot parse key......");
 
    let provider = ProviderBuilder::new()
        .wallet(signer)
        .connect(&env::var("rpc_url").unwrap()).await?;
    let recipient = address!("0x5EE15251C47e60769F2E63605d4323ba54c07983");
    let claim = address!("0x3AC5a5f60753bbfaD93B668A0bEC5c8fA0E647be");
    let claim_instance = MiniFaucet::new(claim, provider); 
    let claimer = claim_instance.claim(recipient); 
    let claim_tx = match claimer.send().await {
        Ok(receipt) => println!("{:?}",receipt.get_receipt().await),
        Err(e) => println!("failed to send: {:?}", e),
    };
    Ok(())
}