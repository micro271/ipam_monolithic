use super::Uuid;
use serde::Deserialize;
use std::net::IpAddr;
#[derive(Deserialize, Debug)]
pub struct ParamDevice {
    pub ip: IpAddr,
    pub network_id: Uuid,
}

#[derive(Deserialize)]
pub struct ParamSubnetting {
   pub father_id: uuid::Uuid,
   pub prefix: u8, 
}