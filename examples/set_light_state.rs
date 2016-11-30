extern crate philips_hue_client;
extern crate regex;

use std::env;
use std::time::Duration;
use regex::Regex;

use philips_hue_client::hue::LightCommand;
use philips_hue_client::bridge::{discover, Bridge};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        println!("usage : {:?} <username> <light_id>,<light_id>,... on|off|[bri]:[hue]:[sat]|[ct]MK:[bri]|[w]K:[bri]|[RR][GG][BB]:[bri]",
                 args[0]);
        return;
    }
    let bridge = Bridge::new(discover().unwrap().pop().unwrap().into_ip(), &*args[1]);
    let ref input_lights: Vec<usize> = args[2].split(",").map(|s| s.parse::<usize>().unwrap()).collect();
    let ref command = args[3];
    let re_triplet = Regex::new("([0-9]{0,3}):([0-9]{0,5}):([0-9]{0,3})").unwrap();
    let re_mired = Regex::new("([0-9]{0,4})MK:([0-9]{0,5})").unwrap();
    let re_kelvin = Regex::new("([0-9]{4,4})K:([0-9]{0,5})").unwrap();
    let re_rrggbb = Regex::new("([0-9a-fA-F]{2})([0-9a-fA-F]{2})([0-9a-fA-F]{2})").unwrap();

    let mut light_command = LightCommand::default();

    let parsed = match &command[..] {
        "on" => light_command.on(),
        "off" => light_command.off(),
        _ if re_triplet.is_match(&command) => {
            let caps = re_triplet.captures(&command).unwrap();

            light_command.bri = caps.at(1).and_then(|s| s.parse::<u8>().ok());
            light_command.hue = caps.at(2).and_then(|s| s.parse::<u16>().ok());
            light_command.sat = caps.at(3).and_then(|s| s.parse::<u8>().ok());
            light_command
        }
        _ if re_mired.is_match(&command) => {
            let caps = re_mired.captures(&command).unwrap();

            light_command.ct = caps.at(1).and_then(|s| s.parse::<u16>().ok());
            light_command.bri = caps.at(2).and_then(|s| s.parse::<u8>().ok());
            light_command.sat = Some(254);
            light_command
        }
        _ if re_kelvin.is_match(&command) => {
            let caps = re_kelvin.captures(&command).unwrap();

            light_command.ct = caps.at(1).and_then(|s| s.parse::<u32>().ok().map(|k| (1000000u32 / k) as u16));
            light_command.bri = caps.at(2).and_then(|s| s.parse::<u8>().ok());
            light_command.sat = Some(254);
            light_command
        }
        _ if re_rrggbb.is_match(&command) => {
            let caps = re_rrggbb.captures(&command).unwrap();

            let rgb: Vec<u8> = [caps.at(1), caps.at(2), caps.at(3)].iter().map(|s| u8::from_str_radix(s.unwrap(), 16).unwrap()).collect();
            let (hue, sat, bri) = rgb_to_hsv(rgb[0], rgb[1], rgb[2]);
            println!("{:?}", (hue, sat, bri));
            light_command.with_hue(hue).with_sat(sat).with_bri(bri)
        }
        _ => panic!("can not understand command {:?}", command),
    };
    let mut bridge_lights = bridge.get_lights().unwrap();

    for light in bridge_lights.iter_mut() {
        if input_lights.contains(&light.id){
            match light.set_state(&bridge, parsed){
                Ok(resps) => for resp in resps.into_iter(){
                    if let Some(success) = resp.success{
                        println!("Success: {:?}", success)
                    }else if let Some(err) = resp.error{
                        println!("Error: {:?}", err);
                    }
                },
                Err(e) => println!("Error happened trying to send request:\n\t{:?}", e)
            }
            std::thread::sleep(Duration::from_millis(50))
        }
    }
}


fn rgb_to_hsv(r: u8, g: u8, b: u8) -> (u16, u8, u8) {
    let r = r as f64 / 255f64;
    let g = g as f64 / 255f64;
    let b = b as f64 / 255f64;
    let max = r.max(g.max(b));
    let min = r.min(g.min(b));

    if max == min {
        (0, 0, (max * 255.) as u8)
    } else {
        let d = max - min;
        let s = d / max;
        let h = if max == r {
            (g - b) / d + (if g < b { 6f64 } else { 0f64 })
        } else if max == g {
            (b - r) / d + 2f64
        } else {
            (r - g) / d + 4f64
        };
        ((65535. * h / 6.) as u16, (s * 255.) as u8, (max * 255.) as u8)
    }
}
