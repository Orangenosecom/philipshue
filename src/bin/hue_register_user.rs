extern crate philipshue;
use std::env;
use std::thread;
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage : {:?} <devicetype>", args[0]);
    } else {
        let mut bridge = None;
        let discovery = philipshue::bridge::discover().unwrap().pop().unwrap();

        for res in discovery.build_bridge().register_user(&*args[1]){
            match res{
                Ok(r) => {
                    bridge = Some(r);
                    break
                },
                Err(e) => {println!("{:?}", e);thread::sleep(Duration::from_secs(2))}
            }
        }
        println!("{:?}", bridge);
    }
}
