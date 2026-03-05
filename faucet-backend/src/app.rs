use crate::handlers::write::claim as handler_claim;
use crate::middleware::rate_limit;
use crate::handlers::read::{faucet_addy, faucet_balance, address_balance, next_claim};
use actix_web::{App, HttpResponse, HttpServer, Responder, get, web::{self, }};
use actix_extensible_rate_limit::{
    backend::{memory::InMemoryBackend}
};

#[actix_web::main]
pub async fn app() -> std::io::Result<()> {
    let backend = InMemoryBackend::builder().build();
    HttpServer::new(move || {
        let limiter = rate_limit(backend.clone());
        App::new().wrap(limiter).service(web::scope("/api").service(handler_claim).service(faucet_addy).service(faucet_balance).service(address_balance).service(next_claim)).service(home)
    })
    .bind(("0.0.0.0", 8080))?.run().await
}

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("WELCOME TO SEPOLIA API FAUCET, BELOW ARE THE VALID ENDPOINTS YOU CAN CALL.\nNOTE: Replace your address to any endpoint where you see \"YOUR-ADDREES\"\n\n/api/claim/YOUR-ADDRESS - use this endpoint to request for faucet every 24hrs \n/api/faucet-address - Use this endpoint to check faucet address if you will like to contribute\n/api/faucet-balance - Use this endpoint to check if there's enough token in the faucet\n/api/my-balance/YOUR-ADDRESS - Use this address to check your own balance\n/api/next-claim-time/YOUR-ADDRESS - Use this endpoint to check your next available claim time(you can only claim once every 24hrs)")
}
