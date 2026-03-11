use crate::imports::*;

#[component]
pub fn claim_input() -> Element  {
    
    let mut result = use_signal(||None);
    let claim = move |_| async move {
    let response = reqwest::get("https://sepolia-faucet.eccentrichealer.xyz/api/next-claim-time/0x34e24FfAAEdF984f90395E0df35929503AA8BE9e")
        .await
        .unwrap()
        .json::<Request>()
        .await
        .unwrap();

        result.set(Some(response.message));
    };

    rsx!{ /*document::Stylesheet { href: WALLET_SUBMIT_CSS},*/
        div { 
            h1 { "Welcome to faucet" }br {}
            // input {  }
            button {onclick: claim, "claim"  }
            if let Some(e) = result() {
                p { "{e}" }
            }
         }


    }
}