extern crate philipshue;
use philipshue::bridge;

#[cfg(feature = "ssdp")]
fn main() {
    let mut ips = bridge::discover_upnp().unwrap();
    ips.dedup();

    println!("Hue bridges found: {:#?}", ips);
}

#[cfg(not(feature = "ssdp"))]
fn main() {
    panic!("Only available with unstable")
}
