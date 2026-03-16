pub mod prelude;
pub mod handlers;
pub mod types;
pub mod components;

fn main() {
    let run_tui = components::tui::run();
    if let Err(error) = run_tui {
        eprintln!("Error: {}", error);
    }
}
