use super::Uuid;
use libipam::type_net::port::Port;
use serde::Deserialize;
use std::net::IpAddr;

#[derive(Deserialize)]
pub struct ParamDevice {
    pub ip: IpAddr,
    pub network_id: Uuid,
}

#[derive(Deserialize)]
pub struct ParamSubnetting {
    pub father_id: uuid::Uuid,
    pub prefix: u8,
}

#[derive(Deserialize)]
pub struct ParamPKService {
    pub port: Port,
    pub ip: IpAddr,
    pub network_id: Uuid,
}

#[derive(Deserialize)]
pub struct ParamPKServiceGet {
    pub port: Option<Port>,
    pub ip: IpAddr,
    pub network_id: Uuid,
}

#[derive(Deserialize)]
pub struct ParamDeviceGet {
    pub ip: Option<IpAddr>,
    pub network_id: Uuid,
}
