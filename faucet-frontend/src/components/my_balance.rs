use crate::imports::*;

#[component]
pub fn MyBalance() -> Element {
    let mut isfetching = use_signal(|| false);
    let mut status = use_signal(|| "".to_string());
    let mut address = use_signal(|| "".to_string());
    let mut user_balance = use_signal(|| None);
    let balance_check = move |_| async move {
        isfetching.set(true);
        let url = format!(
            "https://sepolia-faucet.eccentrichealer.xyz/api/my-balance/{}",
            address()
        );
        let connection = req_get(url).await;
        if let Err(e) = connection {
            user_balance.set(Some(format!("{e}")));
            status.set("error".to_string());
            isfetching.set(false);
            return;
        }
        let result = connection.unwrap();
        let result = result.json::<Request>().await;
        if let Err(e) = result {
            user_balance.set(Some(format!("{e}")));
            status.set("error".to_string());
            isfetching.set(false);
            return;
        }
        let final_result = result.unwrap();
        user_balance.set(Some(final_result.message));
        status.set("success".to_string());
        isfetching.set(false);
    };
    rsx! {
       div {
           h1 { "CHECK YOUR SEPOLIA BALANCE HERE" }br{}br{}
           Link { to: MyRoute::Home, "Go back home"}br{}br{}
           label { "Input your Address below" }br {}
           input { type: "text", oninput: move|i|address.set(i.value()) }br{}
           button { onclick: balance_check, disabled: isfetching() || address.len() != 42 ,
                if isfetching(){ "Fetching ...."}
                else { "Check Balance" } }
           p {
               if isfetching() { "fetching balance...please wait" }
               else{
                   if let Some(s) = user_balance() {
                       if status() == "success" {
                           "Your balance is {s} ETH"
                       }
                       else if status() == "error" {
                           "Error: {s}"
                       }
                       else{ "{s}" }
                   }
               }
           }
       }

    }
}
