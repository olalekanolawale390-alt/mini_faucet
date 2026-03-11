use crate::blockchain::imports::*;

 pub fn rate_limit (backend: InMemoryBackend) -> RateLimiter<InMemoryBackend, actix_extensible_rate_limit::backend::SimpleOutput, impl Fn(&actix_web::dev::ServiceRequest) -> std::future::Ready<Result<actix_extensible_rate_limit::backend::SimpleInput, actix_web::Error>> + 'static> 
 
 {
    let input = SimpleInputFunctionBuilder::new(Duration::from_secs(1), 2)
            .peer_ip_key()
            .build();
        let middleware = RateLimiter::builder(backend.clone(), input)
            .add_headers()
            .build();
        middleware

}


