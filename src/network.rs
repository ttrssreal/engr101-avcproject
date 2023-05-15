use std::net::{TcpStream, SocketAddr};
use std::io::{Write, Read};
use std::str;

use crate::config::GateConfig;

pub struct NetworkGate {
    config: GateConfig,
}

impl NetworkGate {
    pub fn new(config: GateConfig) -> Self {
        Self { config }
    }
    
    pub fn open_gate(&self) -> Result<(), std::io::Error> {
        let mut stream = TcpStream::connect(
            SocketAddr::new(self.config.ip, self.config.port)
        )?;
        let mut resp_buf = [0u8; 6];
        writeln!(&mut stream, "Please\n")?;
        stream.read(&mut resp_buf)?;
        writeln!(&mut stream, "{}\n", str::from_utf8(&resp_buf).unwrap())?;
        Ok(())
    }
}