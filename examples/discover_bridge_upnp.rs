extern crate philipshue;
use philipshue::bridge;

fn main() {
    let mut ips = bridge::discover_upnp().unwrap();
    ips.dedup();

    println!("Hue bridges found: {:#?}", ips);
}
