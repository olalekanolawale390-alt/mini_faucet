use crate::imports::*;

#[component]
pub fn Claim() -> Element {
    let mut status = use_signal(|| format!(""));
    let mut address = use_signal(|| format!(""));
    let mut hash = use_signal(|| None);
    let mut isfetching = use_signal(|| false);
    let claimer = move |_| async move {
        isfetching.set(true);
        let url = format!(
            "https://sepolia-faucet.eccentrichealer.xyz/api/claim/{}",
            address()
        );
        let response = reqwest::get(url).await;
        if let Err(e) = response {
            let error = e.to_string();
            isfetching.set(false);
            status.set("error".to_string());
            return hash.set(Some(error));
        }
        let result = response.unwrap().json::<Request>().await;
        if let Err(e) = result {
            let error = e.to_string();
            isfetching.set(false);
            status.set("error".to_string());
            return hash.set(Some(error));
        }
        let result = result.unwrap();
        if result.status == "error" {
            status.set("error".to_string());
            hash.set(Some(result.message));
            isfetching.set(false);
        } else {
            status.set(result.status);
            hash.set(Some(result.message));
            isfetching.set(false);
        }
    };

    rsx! {
        div {
            h1 { "WELCOME TO THE CLAIM PAGE" }br {} br {}
            Link {to: MyRoute::Home, "Home"}br {}
            label {"enter your address below"}br {}
            input {type: "text", oninput: move |e| address.set(e.value())   }
            br{}
            button {
                onclick: claimer,
                disabled: isfetching() || address.len() != 42,
                 if isfetching() {
                    "⏳ Claiming..."
                 }
                 else {
                    "CLAIM"
                 }
            }
            p {
                if let Some(hash) = hash() {
                    if status() == "success" {
                        "Claim sucessful" br {}
                        "transaction hash: {hash}"
                    }
                    else {

                        "Error: {hash}"
                    }
                }
             }

         }
    }
}
