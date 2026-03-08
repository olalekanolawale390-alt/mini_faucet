use dioxus::prelude::*;
const WALLET_SUBMIT_CSS : Asset = asset!("assets/tailwind.css");

pub fn claim_input() -> Element  {
    rsx!{ document::Stylesheet {href: WALLET_SUBMIT_CSS}
        div {class: "bg-black text-white min-h-screen p-4",
            nav {class: "flex items-center justify-between p-6  bg-neutral-950 text-white",  }
            div { class: "bg-neutral-900 rounded-3xl p-12 shadow-2xl w-full max-w-xl mx-auto mt-60",
                h1 { class: "text-2xl font-extrabold text-center", "WELCOME MINI SEPOLIA FAUCET" }
                p { class: "text-xl text-neutral-400 mb-10 text-center mt-1" , "claim your daily sepolia faucet here" }
                label {class: "block text-lg mb-3 text-neutral-300", "Enter your Address here" }
                input {class: "w-full p-4 rounded-xl border border-neutral-700 bg-neutral-800 text-white placeholder-neutral-500", placeholder:"0x000000000000000000", }
                button {class: "w-full mt-6 p-4 rounded-xl bg-blue-600 text-white font-extrabold font-stretch-200% hover:bg-blue-700 transition", "Submit Wallet"} 
            } 
            footer { class:"","footer" }
        }
    }
}