extern crate philipshue;
extern crate regex;

use std::env;
use std::time::Duration;
use std::num::ParseIntError;

use philipshue::hue::LightCommand;
use philipshue::bridge::{discover, Bridge};

fn main(){
    match run(){
        Ok(()) => (),
        Err(_) => println!("Invalid number!")
    }
}

fn run() -> Result<(), ParseIntError> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        println!("Usage: {} <username> <light_id>,<light_id>,... on|off|bri <bri>|hue <hue>|sat <sat>|rgb <r> <g> <b>|hsv <hue> <sat> <bri>|mired <ct> <bri>|kelvin <temp> <bri>",
                 args[0]);
        return Ok(());
    }
    let bridge = Bridge::new(discover().unwrap().pop().unwrap().into_ip(), &*args[1]);
    let input_lights = args[2].split(",").fold(Ok(Vec::new()), |v, s| v.and_then(|mut v| s.parse::<usize>().map(|n| v.push(n)).map(|_| v)))?;

    let cmd = LightCommand::default();

    let cmd = match &*args[3]{
        "on" => cmd.on(),
        "off" => cmd.off(),
        "bri" => cmd.with_bri(args[4].parse()?),
        "hue" => cmd.with_hue(args[4].parse()?),
        "sat" => cmd.with_sat(args[4].parse()?),
        "hsv" => cmd
            .with_hue(args[4].parse()?)
            .with_sat(args[5].parse()?)
            .with_bri(args[6].parse()?),
        "rgb" => {
            let (hue, sat, bri) = rgb_to_hsv(args[4].parse()?,
                args[5].parse()?, args[6].parse()?);
            cmd.with_hue(hue).with_sat(sat).with_bri(bri)
        },
        "mired" => cmd
            .with_ct(args[4].parse()?)
            .with_bri(args[5].parse()?)
            .with_sat(254),
        "kelvin" => cmd
            .with_ct((1000000u32 / args[4].parse::<u32>()?) as u16)
            .with_bri(args[5].parse()?)
            .with_sat(254),
        _ => panic!("Invalid command!")
    };

    for id in input_lights.into_iter() {
        match bridge.set_light_state(id, &cmd){
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

    Ok(())
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
