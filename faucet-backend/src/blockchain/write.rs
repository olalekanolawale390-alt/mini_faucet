use actix_web::{Responder, web};
use alloy::{sol_types::Revert,primitives::{Address}, providers::ProviderBuilder, signers::local::PrivateKeySigner, sol };
use std::{env};
use serde::{Deserialize, Serialize};
use dotenv::dotenv;

sol! { 
    #[sol(rpc)] 
    contract MiniFaucet { 
         function deposit () public payable;
          function  claim(address _to) public payable;
    } 
}

#[derive(Deserialize, Serialize)]
struct Status{message: String, status: String}

#[tokio::main]
pub async fn claim(address: String) -> impl Responder{
    dotenv().ok();

    let private_key = env::var("PRIVATE_KEY").unwrap();
    let signer: PrivateKeySigner =
        private_key.parse().expect("cannot parse key......");
 
    let provider = ProviderBuilder::new()
        .wallet(signer)
        .connect(&env::var("rpc_url").unwrap()).await.expect("cannotv connect");
    let recipient = address.parse::<Address>();
    if recipient.is_err(){
        if let Err(error) = recipient{
            return web::Json(Status{message: error.to_string(), status: "address_error".to_string()});}
    }
    let recipient = recipient.unwrap();
    let contract = env::var("CONTRACT_ADDRESS").unwrap();
    let claim = contract.parse::<Address>().unwrap();
    let claim_instance = MiniFaucet::new(claim, provider);
    let claimer = claim_instance.claim(recipient);
    let req =  claimer.send()
    .await;
    if req.is_err() {
        if let Err(error) = req {
            let result = error.as_decoded_error::<Revert>();
            let result = result.map(|r|r.to_string()).unwrap_or(error.to_string());
            let result = Status{message: result, status: "error".to_string()};
            return web::Json(result);
        };
    }
    let receipt = req.unwrap().watch().await;
    web::Json(Status{message: format!("tx_hash: {}",receipt.unwrap()), status: format!("success")})
}

