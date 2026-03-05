use actix_web::{get, web, HttpResponse, Responder};
use crate::blockchain::read::{get_faucet_balance, get_wallet_balance, next_claim as nextClaim};

#[get("/faucet-address")]
pub async fn faucet_addy() -> String {
    dotenv::dotenv().ok();
    match std::env::var("CONTRACT_ADDRESS"){
        Ok(ca) => ca,
        Err(e) => e.to_string(),
    } 
}

#[get("/faucet-balance")]
pub async fn faucet_balance() -> impl Responder{
    let balance = tokio::task::spawn_blocking(||{ get_faucet_balance()})
        .await;
    match balance {
        Ok(result) => {
            HttpResponse::Ok().body(result) 
        },
        Err(e) => {
            let error = format!("{}", e);
            HttpResponse::GatewayTimeout().body(error)
        }
    }
}

#[get("/my-balance/{address}")]
pub async fn address_balance(addy: web::Path<String>) -> impl Responder{
    let address = addy.into_inner();
    let wallet_balance = tokio::task::spawn_blocking(|| get_wallet_balance(address)).await;
    match wallet_balance {
        Ok(result) => HttpResponse::Ok().body(result.display()),
        Err(e) => HttpResponse::Ok().body(e.to_string())
    }
}

#[get("/next-claim-time/{address}")]
pub async fn next_claim(address: web::Path<String>) -> impl Responder {
    let address = address.into_inner();
    let next_time = tokio::task::spawn_blocking(|| nextClaim(address));
    match next_time {
        result => result.await.unwrap()
    }
}