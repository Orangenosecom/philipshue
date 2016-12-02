extern crate philips_hue_client;

#[cfg(feature = "ssdp")]
fn main() {
    use philips_hue_client::bridge;
    let mut ips = bridge::discover_upnp().unwrap();
    ips.dedup();

    println!("Hue bridges found: {:#?}", ips);
}

#[cfg(not(feature = "ssdp"))]
fn main() {
    panic!("Only available with unstable")
}
