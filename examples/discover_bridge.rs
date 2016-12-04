extern crate philipshue;
use philipshue::bridge;

fn main() {
    let discoveries = bridge::discover().unwrap();

    println!("Hue bridges found: {:#?}", discoveries);
}
