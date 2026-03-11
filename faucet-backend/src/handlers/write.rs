use crate::blockchain::imports::*;

#[get("/claim/{address}")]
pub async fn claim(address: web::Path<String>) -> impl Responder{
    let address = address.into_inner();
    let result = blockchainClaim(address).await;
    result
}