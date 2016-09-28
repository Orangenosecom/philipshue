extern crate philipshue;
use philipshue::bridge::BridgeBuilder;

fn main() {
    let bridge = BridgeBuilder::discover().unwrap();
    println!("Hue bridge found: {:?}", bridge);
}
