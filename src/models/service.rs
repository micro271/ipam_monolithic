use libipam::type_net::port::Port;
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use uuid::Uuid;

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct Service {
    pub port: Port,
    pub ip: IpAddr,
    pub netwok_id: Uuid,
    pub service_id: Uuid,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct ServiceUpdate {
    pub port: Option<Port>,
    pub ip: Option<IpAddr>,
    pub netwok_id: Option<Uuid>,
    pub service_id: Option<Uuid>,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Services {
    pub id: Uuid,
    pub name: String,
    pub version: String,
}

#[derive(Deserialize, Debug)]
pub struct ServicesUpdate {
    pub name: Option<String>,
    pub version: Option<String>,
}
