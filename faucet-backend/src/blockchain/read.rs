use alloy::{primitives::{Address, U256, address}, providers::ProviderBuilder, sol, sol_types::{RevertReason, Revert}};
use actix_web::{HttpRequest,HttpResponse,Responder, body::BoxBody, web};
use std::{env, string};
use serde::{Serialize};
use dotenv::dotenv;
use chrono::{DateTime, Utc};

#[derive(Serialize)]
enum MyResponse<E,S,/*W*/>{
    Success(S),
    Error(E),
    //IncorrectAddr(W)
}

impl <E,S> Responder for MyResponse<E,S>
where 
    E: serde::Serialize,
    S: serde::Serialize {
        type Body = BoxBody;

        fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
            match self {
                MyResponse::Success(success) => HttpResponse::Ok().json(success),
                MyResponse::Error(error) => HttpResponse::ExpectationFailed().json(error),
            }
        }
    }

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
pub async fn get_faucet_balance() -> impl Responder{
    dotenv().ok();
    let provider = ProviderBuilder::new().connect(&env::var("rpc_url").unwrap()).await;
    let faucet_balance = address!("0x3AC5a5f60753bbfaD93B668A0bEC5c8fA0E647be");
    let faucet_balance_instance = MiniFaucet::new(faucet_balance, provider.unwrap());
    let get_faucet_balance = faucet_balance_instance.faucetBalance().call().await;
  
    if let Err(e) = get_faucet_balance {
        let error = e.as_decoded_error::<Revert>();
        let error = error.map(|r|format!("{r}")).unwrap_or(format!("server unable to connect to blockchain"));
        let error = MyResponse::Error(error);
        return web::Json(error);
    } 
    let success = get_faucet_balance.ok().unwrap();
    let success = MyResponse::Success(success);
    web::Json(success)



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

#[derive(Serialize)]
struct Status{message: String, status: String}
#[tokio::main]
pub async fn next_claim(address: String) -> impl Responder {
    dotenv().ok();
    let user = address.parse::<Address>();
    if let Err(error) = user {let message = error.to_string(); return web::Json(Status{ message, status: String::from("error") });}
    let user = user.unwrap();
    let provider  = ProviderBuilder::new().connect(&env::var("rpc_url").unwrap()).await.unwrap();
    let next_claim = address!("0x3AC5a5f60753bbfaD93B668A0bEC5c8fA0E647be");
    let next_claim_instance = MiniFaucet::new(next_claim, provider);
    let req = next_claim_instance.nextClaimTime(user).call().await ;

    if let Err(err) = req {
        let reason = err.as_decoded_error::<Revert>();
        let reason = reason.map(|r|r.to_string()).unwrap_or(format!("failed to connect to blockchain, check your connection and try again"));
        let result = Status {message: reason, status: "error".to_string()};
        return web::Json(result);
    }

    let get_next_claim = req.unwrap().to_string();
    let get_next_claim = get_next_claim.parse::<i64>().unwrap();
    let dt: DateTime<Utc> = DateTime::from_timestamp_secs(get_next_claim).unwrap();
    let dt = dt.to_rfc2822();
    web::Json(Status { message: dt, status: String::from("success") })
}