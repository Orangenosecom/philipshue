extern crate philipshue;

#[cfg(feature = "ssdp")]
fn main() {
    use philipshue::bridge;
    let mut ips = bridge::discover_upnp().unwrap();
    ips.dedup();

    println!("Hue bridges found: {:#?}", ips);
}

#[cfg(not(feature = "ssdp"))]
fn main() {
    panic!("Only available with unstable")
}
