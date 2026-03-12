use crate::imports::*;

#[component]
pub fn FaucetBalance() -> Element {
    let mut ischecking = use_signal(|| false);
    let mut status = use_signal(|| "".to_string());
    let mut getbalance = use_signal(|| None);
    let fetcher = move |_| async move {
        ischecking.set(true);
        let url = "https://sepolia-faucet.eccentrichealer.xyz/api/faucet-balance".to_string();
        let connection = req_get(url).await;
        if let Err(e) = connection {
            let error = e.to_string();
            getbalance.set(Some(error));
            status.set("error".to_string());
            ischecking.set(false);
            return;
        }

        let connection_result = connection.unwrap();
        let json_parser = connection_result.json::<Request>().await;
        if let Err(e) = json_parser {
            let error = e.to_string();
            status.set("error".to_string());
            getbalance.set(Some(error));
            ischecking.set(false);
            return;
        }

        let json_parse_result = json_parser.unwrap();
        if json_parse_result.status == "success" {
            getbalance.set(Some(json_parse_result.message));
            status.set("success".to_string());
            ischecking.set(false);
        } else if json_parse_result.status == "error" {
            getbalance.set(Some(json_parse_result.message));
            status.set("error".to_string());
            ischecking.set(false);
        } else {
            getbalance.set(None);
            ischecking.set(false);
        }
        ischecking.set(false);
        ischecking.set(false);
    };

    rsx! {

        div {
            h1 {
                "CHECK TO SEE IF THERE'S ENOUGH TOKEN LEFT"
            }br{}br{}

            Link { to:MyRoute::Home, "Go Home"} br{}

            button {
                onclick: fetcher, disabled: ischecking,
                if ischecking() {
                "......Checking......."
                }
                else {"Click to check faucet balance"}
            }
            p {
                if ischecking() {
                "Processing Balance check"
                }
                else{
                    if let Some(balance) = getbalance() {
                        if status() == "success" {
                        "successfully checked, Faucet balance is {balance} ETH"
                        }
                        else {
                            "Error fetching balance" br{}
                            "Reason {balance}"
                        }
                    }
                }
            }
        }
    }
}
