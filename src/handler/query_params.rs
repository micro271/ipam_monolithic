use super::Uuid;
use serde::Deserialize;
use std::net::IpAddr;
use libipam::type_net::port::Port;

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