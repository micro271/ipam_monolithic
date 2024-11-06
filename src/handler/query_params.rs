use serde::{Deserialize, Serialize};
use super::Uuid;
use std::net::IpAddr;

#[derive(Deserialize, Serialize, Debug)]
pub struct ParamDevice {
    pub ip: IpAddr,
    pub network_id: Uuid,
}