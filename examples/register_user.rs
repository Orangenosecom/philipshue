extern crate philipshue;

use std::env;
use std::thread;
use std::time::Duration;

use philipshue::bridge;
use philipshue::errors::{HueError, BridgeError};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage : {:?} <devicetype>", args[0]);
    } else {
        let discovery = philipshue::bridge::discover().unwrap().pop().unwrap();
        let ip = discovery.ip();

        loop{
            match bridge::register_user(&ip, &*args[1]){
                Ok(bridge) => {
                    println!("User registered: {}, on IP: {}", bridge, ip);
                    break;
                },
                Err(HueError::BridgeError{error: BridgeError::LinkButtonNotPressed, ..}) => {
                    println!("Please, press the link on the bridge. Retrying in 5 seconds");
                    thread::sleep(Duration::from_secs(5));
                }
                Err(e) =>{
                    println!("Unexpected error occured: {:?}", e);
                    return
                }
            }
        }
    }
}
