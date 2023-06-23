use std::{
    cmp,
    error::Error,
    io::{self, Write},
};

use crate::light::controller::{print_controllers, Controller};

fn read_user_input() -> Result<String, io::Error> {
    let mut raw_input = String::new();
    print!("> ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut raw_input)?;
    Ok(raw_input)
}

fn handle_user_input(
    controllers: &Vec<Controller>,
    raw_input: String,
) -> Result<(usize, u64), Box<dyn Error>> {
    let parts = raw_input.trim().split(':').collect::<Vec<_>>();
    if parts.len() != 2 {
        return Err("bad format".into());
    }

    let max = controllers.len();

    let controller_id = parts[0].parse::<usize>()?;
    if controller_id >= max {
        return Err("bad controller_id".into());
    }

    Ok((
        controller_id,
        cmp::min::<u64>(
            parts[1].parse::<u64>()?,
            controllers[controller_id].max_brightness,
        ),
    ))
}

pub fn run(controllers: &mut Vec<Controller>) -> Result<(), Box<dyn std::error::Error>> {
    print_controllers(&controllers);
    loop {
        let Ok(user_input) = read_user_input() else {
            break
        };
        if let Ok((cid, n)) = handle_user_input(&controllers, user_input) {
            print!(
                "update controller {} with value {}\n",
                &controllers[cid].path.display(),
                n
            );
            if let Err(err) = controllers[cid].set_brightness(n) {
                print!("{}\n", err);
                break;
            };
            print_controllers(&controllers);
        } else {
            println!("Use the following format: <list number>:<value>");
            println!("Example: 4:255");
            continue;
        }
    }

    Ok(())
}
