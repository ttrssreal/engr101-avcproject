use std::net::TcpStream;
use std::io::{Write, Read};
use std::str;

use crate::config::GateConfig;

pub struct Gate {
    config: GateConfig,
}

impl Gate {
    pub fn new(config: GateConfig) -> Self {
        Self { config }
    }
    
    pub fn open_gate(&self) -> Result<(), std::io::Error> {
        let mut stream: TcpStream = TcpStream::connect(self.config.sock_addr.sock)?;
        let mut resp_buf = [0u8; 6];
        writeln!(&mut stream, "Please\n")?;
        stream.read(&mut resp_buf)?;
        writeln!(&mut stream, "{}\n", str::from_utf8(&resp_buf).unwrap())?;
        Ok(())
    }
}
