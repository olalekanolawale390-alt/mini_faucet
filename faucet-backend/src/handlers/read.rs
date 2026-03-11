use crate::blockchain::imports::*;

#[get("/faucet-address")]
pub async fn faucet_addy() -> impl Responder {
    dotenv::dotenv().ok();
    match std::env::var("CONTRACT_ADDRESS"){
        Ok(ca) => MyResponse::<Value,Value, Value>::Success(json!({"status" : "success", "message" : ca})),
        Err(e) => MyResponse::Error(json!({
            "status" : "error", "message" : format!("{e}")
        })),
    } 
}


#[get("/faucet-balance")]
pub async fn faucet_balance() -> impl Responder{
    get_faucet_balance().await
}

#[get("/my-balance/{address}")]
pub async fn address_balance(addy: web::Path<String>) -> impl Responder{
    let address = addy.into_inner();
    get_wallet_balance(address).await
}


#[get("/next-claim-time/{address}")]
pub async fn next_claim(address: web::Path<String>) -> impl Responder {
    let address = address.into_inner();
    nextClaim(address).await
}