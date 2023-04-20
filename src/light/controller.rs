use std::error::Error;
use std::path::{Path, PathBuf};
use std::{fmt, fs, io};

use crate::utils::read_file_contents;

pub struct Controller {
    pub path: PathBuf,
    pub brightness: u64,
    pub max_brightness: u64,
}

impl Controller {
    pub fn new(controller_path: PathBuf) -> Result<Controller, Box<dyn Error>> {
        if !controller_path.exists() {
            return Err("path does not exists".into());
        }

        let actual_brightness = read_file_contents(controller_path.join("brightness"))?;
        let max_brightness = read_file_contents(controller_path.join("max_brightness"))?;

        Ok(Controller {
            path: controller_path,
            brightness: actual_brightness,
            max_brightness: max_brightness,
        })
    }

    pub fn set_brightness(&self, new_brightness: u64) -> io::Result<()> {
        fs::write(self.path.join("brightness"), new_brightness.to_string())
    }
}

impl fmt::Display for Controller {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} -> {} / {}",
            self.path.display(),
            self.brightness,
            self.max_brightness
        )
    }
}

pub fn discover_controllers(paths: Vec<&str>) -> Vec<Controller> {
    let mut controllers = vec![];
    for path in paths {
        let Ok(entries) = fs::read_dir(Path::new(path)) else { continue ; };
        for entry in entries {
            let Ok(entry) = entry else { continue };
            let Ok(controller) = Controller::new(entry.path().to_owned()) else { continue; };
            controllers.push(controller);
        }
    }

    controllers
}

pub fn update_controller(c: &Controller, new_brightness: u64) -> io::Result<()> {
    // let old = c.brightness;
    c.set_brightness(new_brightness)?;
    // thread::sleep(Duration::from_secs(2));
    // c.set_brightness(old)?;

    Ok(())
}

pub fn print_controllers(controllers: &Vec<Controller>) {
    for (i, c) in controllers.into_iter().enumerate() {
        print!("[{:02}] {}\n", i, c);
    }
}
