use std::io;
mod prelude;
mod claim;

fn main() {
    let claimer = claim::claim();
    println!("{:?}", claimer);
    println!("\n\nClick enter to exit");
    io::stdin().read_line(&mut String::new()).expect("end");
}
