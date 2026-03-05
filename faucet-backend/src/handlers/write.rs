use crate::blockchain::write::claim as blockchainClaim;
use actix_web::{get, web, HttpResponse, Responder,};

#[get("/claim/{address}")]
pub async fn claim(address: web::Path<String>) -> impl Responder{
    let address = address.into_inner();
    let claim = tokio::task::spawn_blocking(||blockchainClaim(address)).await;
    // let claim = match claim {
    //     Ok(success) => Ok(success),
    //     Err(error) => Err(error)
    // };
    if claim.is_err() {
        if let Err(error) = claim{return HttpResponse::Forbidden().body(error.to_string())}
    }
    match claim {
        Ok(success) => HttpResponse::Accepted().body(success),
        Err(_e) => todo!()
    }
    

}