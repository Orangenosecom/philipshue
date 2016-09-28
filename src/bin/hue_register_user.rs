extern crate philipshue;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage : {:?} <devicetype>", args[0]);
    } else {
        let bridge = ::philipshue::bridge::BridgeBuilder::discover().unwrap().register_user(&*args[1]);
        println!("{:?}", bridge);
    }
}
