use super::*;
use std::net::IpAddr;

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct UpdateDevice {
    pub ip: Option<IpAddr>,
    pub description: Option<String>,
    pub location: Option<String>,
    pub network_id: Option<Uuid>,
    pub credential: Option<Credential>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Device {
    pub ip: IpAddr,
    pub description: Option<String>,
    pub location: Option<Uuid>,
    pub status: Status,
    pub network_id: uuid::Uuid,
    pub credential: Option<Credential>,
}

impl std::cmp::PartialEq<IpAddr> for Device {
    fn eq(&self, other: &IpAddr) -> bool {
        self.ip.eq(other)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Credential {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::Type, PartialEq)]
pub enum Status {
    Reserved,
    Unknown,
    Online,
    Offline,
}

impl Default for Status {
    fn default() -> Self {
        Self::Unknown
    }
}
