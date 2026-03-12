use crate::imports::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            h1 {
                "WELCOME TO SEPOLIA FAUCET"
             }br {}
             Link {to: MyRoute::Claim, "Claim page"} br {}
             button {
                onclick: move |_|{ use_navigator().push(MyRoute::NextTime);}, "check next claim time"
              }br {  }
              Link{ to: MyRoute::MyBalance, "check balance"} br{}br{}
              Link{to: MyRoute::FaucetBalance, "Faucet Balance"} br{}br{}
              Link{to: MyRoute::Contribute, "Contribute"}

         }
    }
}
