mod bcm2835;
mod network;
mod config;

use crate::{config::SystemConfig, network::NetworkGate};

fn main() -> Result<(), std::io::Error> {
    println!("ENG101 - AVC Project. Team: 28");
    bcm2835::init(0);

    let config_str: String = std::fs::read_to_string("system-config.yaml")?;
    let system_config: SystemConfig = serde_yaml::from_str(&config_str).unwrap();

    let gate = NetworkGate::new(system_config.gate);

    match gate.open_gate() {
        Ok(_) => println!("Gate opened!"),
        Err(err) => { println!("Failed to open gate!"); return Err(err); },
    };
    
    bcm2835::stoph();
    Ok(())
}