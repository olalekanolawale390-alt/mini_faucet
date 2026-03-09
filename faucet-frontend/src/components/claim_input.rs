use dioxus::prelude::*;
const WALLET_SUBMIT_CSS : Asset = asset!("assets/tailwind.css");

pub fn claim_input() -> Element  {

    let claim = move |evt|{};

    rsx!{ document::Stylesheet { href: WALLET_SUBMIT_CSS},
        div {class: "bg-gray-950 text-white min-h-screen bg-blend-overlay",
            nav { class: "bg-gray-400 p-6", }
            div{class: "bg-slate-500 mx-auto mt-60 max-w-2xl p-12 mask-t-from-0.2
                rounded-3xl h-full mask-t-from-0.5",
                h1 {class: "text-center text-2xl font-extrabold
                    font-stretch-200%","WELCOME TO SEPOLIA MINI FAUCET" }br {  }
                label {class:" after:content-['*'] after:text-red-500 
                    after:ml-1
                    text-lg text-neutral-400 font-bold", "Enter your Address below" }br {}
                input { class:"border-4 border-dotted border-blue-700 w-full
                    p-4 mb-4 rounded-xl",placeholder:"0x07ac47d64cb44cd0c7d7557be5fc02b530bd2d10" }br{}
                button { onclick: claim,  class:" p-4 rounded-xl bg-indigo-700 w-full
                    hover:bg-red-400 hover:duration-500 transition duration-1000 ", "Submit Wallet"}
            }
        }

    }
}