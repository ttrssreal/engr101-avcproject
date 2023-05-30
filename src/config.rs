use serde::{Serialize, Deserialize};
use std::net::SocketAddr;
use std::net::ToSocketAddrs;

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct PidConfig {
    pub kp: f32,
    pub ki: f32,
    pub kd: f32,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct GateConfig {
    pub sock_addr: SocketAddrWithDefault,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct CameraConfig {
    pub frame_width: u32,
    pub frame_height: u32,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct SystemConfig {
    pub pid: PidConfig,
    pub gate: GateConfig,
    pub camera: CameraConfig,
    pub debug: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct SocketAddrWithDefault { pub sock: SocketAddr }

impl Default for SocketAddrWithDefault {
    fn default() -> Self {
        SocketAddrWithDefault { sock: ("1.2.3.4", 69)
            .to_socket_addrs()
            .unwrap()
            .next()
            .unwrap()
        }
    }
}
