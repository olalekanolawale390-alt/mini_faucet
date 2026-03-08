use dioxus;
mod components;
use crate::components::claim_input::claim_input;

fn main() {
    dioxus::launch(claim_input);
}