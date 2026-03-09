use crate::blockchain::write::claim as blockchainClaim;
use actix_web::{get, web, Responder,};

#[get("/claim/{address}")]
pub async fn claim(address: web::Path<String>) -> impl Responder{
    let address = address.into_inner();
    let claim = tokio::task::spawn_blocking(||blockchainClaim(address));
    let result = match claim {
        claim => claim.await.unwrap()
    };
    result

}