use sqlx::{sqlite::SqliteRow, Row};
use crate::models::{device::*, network::{Network, Vlan}, user::*, office::Office};
use uuid::Uuid;

impl From<SqliteRow> for Device {
    fn from(value: SqliteRow) -> Self {
        Self {
            ip: value.get::<'_, &str, _>("ip").parse().unwrap(),
            description: value.get("description"),
            office_id: value.get("office_ids"),
            rack: value.get("rack"),
            credential: bincode::deserialize(value.get("credential")).unwrap(),
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