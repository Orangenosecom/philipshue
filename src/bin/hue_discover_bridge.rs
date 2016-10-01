extern crate philipshue;
use philipshue::bridge;

fn main() {
    let discovery = bridge::discover().unwrap().pop().unwrap();
    let bridge = discovery.build_bridge();
    println!("Hue bridge found: {:?}", bridge);
}
