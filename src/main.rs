mod light;
mod utils;

use crate::light::controller::discover_controllers;
use crate::light::ui::dummy::run as dummy_run;
use crate::light::ui::tui::run as tui_run;

use std::env;

fn usage() -> Result<(), Box<dyn std::error::Error>> {
    println!("Lightmixer -- adjust backlight brightness");
    println!("");
    println!("Usage: lightmixer [MODE]");
    println!("");
    println!("Modes:");
    println!("  tui ........... The default terminal user interface mode.");
    println!("                  navigate using the HJKL keys, quit with q");
    println!("  dummy ......... The console interactive mode.");
    println!("");
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let mut controllers = discover_controllers(vec!["/sys/class/backlight", "/sys/class/leds"]);
    if args.iter().any(|e: &String| e == "tui") {
        return tui_run(controllers);
    }

    if args.iter().any(|e: &String| e == "dummy") {
        return dummy_run(&mut controllers);
    }

    return usage();
}
