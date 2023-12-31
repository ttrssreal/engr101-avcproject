use avcproject_team_28::system::System;
use avcproject_team_28::config::SystemConfig;

use std::env::args;
use std::process::exit;

fn main() -> Result<(), std::io::Error> {
    println!("ENG101 - AVC Project. Team: 28");

    let config_fname = args().nth(1).unwrap_or_else(|| {
        println!("Usage: {} <config file>", args().next().unwrap());
        exit(1);
    });

    let config_str: String = std::fs::read_to_string(config_fname)?;
    let system_config: SystemConfig = serde_yaml::from_str(&config_str).unwrap();

    println!("{:#?}", system_config);

    let mut system = System::new(system_config);

    loop {
        match system.update() {
            Ok(finished) => if finished { return Ok(()) },
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            },
        }
    }
    system.stop();
    // wait for hw exchange
    std::thread::sleep(std::time::Duration::from_millis(500));
    Ok(())
}
