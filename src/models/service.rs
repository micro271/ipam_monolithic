use std::net::IpAddr;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use libipam::type_net::port::Port;


#[derive(Serialize, Debug, Deserialize)]
pub struct Service {
    pub port: Port,
    pub ip: IpAddr,
    pub netwok_id: Uuid,
    pub service_id: Uuid,
    pub description: String,
    pub r#type: Type,
}

#[derive(Debug, Deserialize)]
pub struct ServiceUpdate {
    pub port: Option<Port>,
    pub ip: Option<IpAddr>,
    pub netwok_id: Option<Uuid>,
    pub service_id: Option<Uuid>,
    pub description: Option<String>,
    pub r#type: Option<Type>,
}

#[derive(Serialize, Debug, Deserialize)]
pub enum Type {
    Local,
    Contianer,
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Local => write!(f, "Local"),
            Self::Contianer => write!(f, "Container"),
        }
    }
}

#[derive(Debug)]
pub struct ParseStrError;

impl std::fmt::Display for ParseStrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The conversion from str to type failed")
    }
}

impl std::error::Error for ParseStrError {}

impl std::str::FromStr for Type {
    type Err = ParseStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Container" => Ok(Self::Contianer),
            "Local" => Ok(Self::Local),
            _ => Err(ParseStrError)
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
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