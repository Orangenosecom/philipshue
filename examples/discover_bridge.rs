extern crate philips_hue_client;
use philips_hue_client::bridge;

fn main() {
    let discoveries = bridge::discover().unwrap();

    println!("Hue bridges found: {:#?}", discoveries);
}
