use dioxus::prelude::*;
const WALLET_SUBMIT_CSS : Asset = asset!("/assets/wallet_submit.css");

pub fn claim_input() -> Element  {
    rsx!{ document::Stylesheet {href: WALLET_SUBMIT_CSS}
        body { class: "background",
            h1 { "WELCOME TO MINI SEPOLIA FAUCET" }
            div { 
                span { "Enter your sepolia wallet Below"}
                br{}br{}
                input {oninput: move |address| { println!("{}", address.value())}  , placeholder: "enter your wallet address here" }br{}
                button { "Submit Wallet"}
            }
        }
    }
}