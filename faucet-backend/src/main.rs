// use std::io;
mod blockchain;
mod handlers;
mod app;
mod middleware;
fn main(){
    let _app = app::app();
}

