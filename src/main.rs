mod light;
mod utils;

use crate::light::controller::discover_controllers;
use crate::light::ui::dummy::run as dummy_run;
use crate::light::ui::tui::run as tui_run;

use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let controllers = discover_controllers(vec!["/sys/class/backlight", "/sys/class/leds"]);
    if args.iter().any(|e: &String| e == "dummy") {
        return dummy_run(controllers);
    }

    tui_run(controllers)
}
