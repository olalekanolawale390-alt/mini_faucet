use crate::imports::*;

#[component]
pub fn Contribute() -> Element {
    let mut faucet_address = use_signal(|| None);
    let mut status = use_signal(|| "".to_string());
    let mut isfetching = use_signal(|| false);

    let fetcher = move |_| async move {
        isfetching.set(true);
        let url = "https://sepolia-faucet.eccentrichealer.xyz/api/faucet-address".to_string();
        let connection = req_get(url).await;
        if let Err(e) = connection {
            isfetching.set(false);
            faucet_address.set(Some(e.to_string()));
            status.set("error".to_string());
            return;
        }
        let connection_result = connection.unwrap();
        let json_parser = connection_result.json::<Request>().await;
        if let Err(e) = json_parser {
            isfetching.set(false);
            faucet_address.set(Some(e.to_string()));
            status.set("error".to_string());
            return;
        }
        let json_result = json_parser.unwrap();
        if json_result.status == "error" {
            isfetching.set(false);
            faucet_address.set(Some(json_result.message));
            status.set("error".to_string());
        } else if json_result.status == "success" {
            faucet_address.set(Some(json_result.message));
            status.set(json_result.status);
            isfetching.set(false);
        } else {
            faucet_address.set(None);
            isfetching.set(false);
        }
    };

    rsx! {
        div {
            h1 {
                "Click the button below if you will like to contribute"
            }
            Link { to: MyRoute::Home, "Go home"}br{}
            button {onclick: fetcher, disabled: isfetching(),
                if isfetching() {
                    "Fetching...wait..."
                }else {
                    "Check faucet address"
                }
            }
            p {
                if isfetching() {
                    "Fetching faucet address, please wait a few seconds ...."
                }
                else {
                    if let Some(data) = faucet_address() {
                        if status() == "success" {
                            "Succesfully fetched" br{}
                            "Faucet Address is: {data}"
                        }
                        else{
                            "Unable to fetch faucet Address" br{}
                            "Reason: {data}"
                        }
                    }
                }
            }

        }
    }
}
