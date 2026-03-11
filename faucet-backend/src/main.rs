mod blockchain;
mod handlers;
mod app;
mod middleware;
pub mod cors;

fn main(){
    let _app = app::app();
}

