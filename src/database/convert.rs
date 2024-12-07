use crate::models::{
    device::*,
    network::{Network, Vlan},
    office::Office,
    service::{Service, Services},
    user::*,
};
use libipam::type_net::port::Port;
use sqlx::{sqlite::SqliteRow, Row};

impl From<SqliteRow> for Device {
    fn from(value: SqliteRow) -> Self {
        Self {
            ip: value.get::<'_, &str, _>("ip").parse().unwrap(),
            description: value.get("description"),
            location: value.get("location"),
            credential: value
                .get::<'_, Option<Vec<u8>>, _>("credential")
                .map(|x| bincode::deserialize::<'_, Credential>(&x).unwrap()),
            status: value.get("status"),
            network_id: value.get("network_id"),
        }
    }
}

impl From<SqliteRow> for Network {
    fn from(value: SqliteRow) -> Self {
        Self {
            id: value.get("id"),
            description: value.get("description"),
            network: value.get::<'_, &str, _>("network").parse().unwrap(),
            available: value.get::<'_, u32, _>("available").into(),
            used: value.get::<'_, u32, _>("used").into(),
            vlan: Some(Vlan::new(value.get::<'_, i32, _>("vlan") as u16).unwrap()),
            free: value.get::<'_, u32, _>("free").into(),
            father: value.get("father"),
        }
    }
}

impl From<SqliteRow> for Office {
    fn from(value: SqliteRow) -> Self {
        Self {
            id: value.get("id"),
            name: value.get("name"),
            address: value.get("address"),
            description: value.get("description"),
        }
    }
}

impl From<SqliteRow> for User {
    fn from(value: SqliteRow) -> Self {
        Self {
            id: value.get("id"),
            username: value.get("username"),
            password: value.get("password"),
            role: value.get("role"),
        }
    }
}

impl From<SqliteRow> for Service {
    fn from(value: SqliteRow) -> Self {
        Self {
            port: Port::new(value.get("port")),
            ip: value.get::<'_, &str, &str>("ip").parse().unwrap(),
            netwok_id: value.get("network_id"),
            service_id: value.get("service_id"),
            description: value.get("description"),
        }
    }
}

impl From<SqliteRow> for Services {
    fn from(value: SqliteRow) -> Self {
        Self {
            id: value.get("id"),
            name: value.get("name"),
            version: value.get("version"),
        }
    }
}
