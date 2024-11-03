pub mod network;
pub mod utils;

use network::*;
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqliteRow, Row};
use std::{collections::HashMap, net::IpAddr};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
pub struct UpdateDevice {
    pub ip: Option<IpAddr>,
    pub description: Option<String>,
    pub office_id: Option<Uuid>,
    pub rack: Option<Option<String>>,
    pub room: Option<Option<String>>,
    pub status: Option<Status>,
    pub network_id: Option<Uuid>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Device {
    pub ip: IpAddr,
    pub description: Option<String>,
    pub office_id: Option<Uuid>,
    pub rack: Option<String>,
    pub room: Option<String>,
    pub status: Status,
    pub network_id: uuid::Uuid,
    pub credential: Option<Credential>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Credential {
    username: String,
    password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Office {
    pub id: Uuid,
    pub name: String,
    pub address: Option<String>,
    pub description: Option<String>,
}

pub struct UpdateOffice {
    description: Option<String>,
    address: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::Type, PartialEq)]
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

impl From<SqliteRow> for Device {
    fn from(value: SqliteRow) -> Self {
        Self {
            ip: value.get::<'_, &str, _>("ip").parse().unwrap(),
            description: value.get("description"),
            office_id: value.get("office_ids"),
            rack: value.get("rack"),
            credential: {
                let username: String = value.get("username");
                let password: String = value.get("password");

                if username.is_empty() && password.is_empty() {
                    None
                } else {
                    Some(Credential { username, password })
                }
            },
            room: value.get("room"),
            status: value.get("status"),
            network_id: Uuid::parse_str(value.get("network_status")).unwrap(),
        }
    }
}

impl From<SqliteRow> for Network {
    fn from(value: SqliteRow) -> Self {
        Self {
            id: value.get("id"),
            description: value.get("description"),
            network: value.get::<'_, &str, _>("network").parse().unwrap(),
            available: value.get::<'_, i32, &str>("available") as u32,
            used: value.get::<'_, i32, _>("used") as u32,
            total: value.get::<'_, i32, _>("total") as u32,
            vlan: Some(Vlan::new(value.get::<'_, i32, _>("vlan") as u16).unwrap()),
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
