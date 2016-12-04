extern crate philipshue;
use std::env;
use philipshue::bridge::{discover, Bridge};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("usage : {:?} <username>", args[0]);
        return;
    }
    let bridge = Bridge::new(discover().unwrap().pop().unwrap().into_ip(), &*args[1]);

    match bridge.get_all_lights() {
        Ok(lights) => {
            let max_name_len = std::cmp::max(4, lights.values().map(|l| l.name.len()).max().unwrap_or(4));
            println!("id {0:1$} on  bri hue   sat temp  alert   effect    colormode reachable xy", "name", max_name_len);
            for (id, light) in lights.iter() {
                println!("{:2} {:name_len$} {:3} {:3} {:5} {:3} {:4}K {:7} {:9} {:9} {:8} {:?}",
                         id,
                         light.name,
                         if light.state.on { "on" } else { "off" },
                         light.state.bri,
                         Show(&light.state.hue),
                         Show(&light.state.sat),
                         Show(&light.state.ct.map(|ct| 1000000u32 / ct as u32)),
                         light.state.alert,
                         Show(&light.state.effect),
                         Show(&light.state.colormode),
                         light.state.reachable,
                         Show(&light.state.xy),
                         name_len = max_name_len);
            }
        }
        Err(err) => panic!("{:?}", err),
    }
}

use std::fmt::{self, Display, Debug};

struct Show<'a, T: 'a>(&'a Option<T>);

impl<'a, T: 'a + Display> Display for Show<'a, T>{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match *self.0{
            Some(ref x) => x.fmt(f),
            _ => Display::fmt("N/A", f)
        }
    }
}

impl<'a, T: 'a + Debug> Debug for Show<'a, T>{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match *self.0{
            Some(ref x) => x.fmt(f),
            _ => Display::fmt("N/A", f)
        }
    }
}
