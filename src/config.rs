use serde::{Serialize, Deserialize};
use std::net::IpAddr;

#[derive(Serialize, Deserialize, Debug)]
pub struct PidConfig {
    pub prop: i32,
    pub int: i32,
    pub der: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GateConfig {
    pub ip: IpAddr,
    pub port: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SystemConfig {
    pub pid: PidConfig,
    pub gate: GateConfig
}