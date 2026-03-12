pub use dioxus::prelude::*;
pub use reqwest::get as req_get;
pub use serde::Deserialize;

pub use crate::components::{
    claim::Claim, contribute::Contribute, faucet_balance::FaucetBalance, home::Home,
    my_balance::MyBalance, next_time::NextTime,
};
pub use crate::custom::{MyRoute, Request};

// pub const WALLET_SUBMIT_CSS : Asset = asset!("assets/tailwind.css");
