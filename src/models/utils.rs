use super::device::*;
use super::{network::*, *};
use ipnet::IpNet;
use std::{
    collections::HashMap,
    {net::IpAddr, vec},
};
use uuid::Uuid;

pub trait Table {
    fn name() -> String;
    fn query_insert() -> String;
    fn get_fields(self) -> Vec<TypeTable>;
    fn columns() -> Vec<&'static str>;
}

pub trait Updatable<'a> {
    fn get_pair(self) -> Option<HashMap<&'a str, TypeTable>>;
}

impl Table for Device {
    fn columns() -> Vec<&'static str> {
        vec![
            "ip",
            "description",
            "office_id",
            "rack",
            "room",
            "status",
            "network_id",
            "credential",
        ]
    }

    fn name() -> String {
        String::from("devices")
    }

    fn query_insert() -> String {
        format!("INSERT INTO {} (ip, network_id, description, office_id, rack, room, status, credential) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)", Self::name())
    }

    fn get_fields(self) -> Vec<TypeTable> {
        vec![
            self.ip.into(),
            self.network_id.into(),
            self.description.into(),
            self.office_id.into(),
            self.rack.into(),
            self.room.into(),
            self.status.into(),
            self.credential.into(),
        ]
    }
}

impl Table for Network {
    fn columns() -> Vec<&'static str> {
        vec![
            "id",
            "network",
            "available",
            "used",
            "vlan",
            "description",
        ]
    }

    fn name() -> String {
        String::from("networks")
    }

    fn query_insert() -> String {
        format!(
            "INSERT INTO {} (id, network, available, used, vlan, description) VALUES ($1, $2, $3, $4, $5, $6)",
            Self::name()
        )
    }

    fn get_fields(self) -> Vec<TypeTable> {
        vec![
            self.id.into(),
            self.network.into(),
            self.available.into(),
            self.used.into(),
            self.vlan.into(),
            self.description.into(),
        ]
    }
}

impl Table for office::Office {
    fn name() -> String {
        String::from("offices")
    }

    fn query_insert() -> String {
        format!(
            "INSERT INTO {} (id, description, address) VALUES ($1, $2, $3)",
            office::Office::name()
        )
    }

    fn get_fields(self) -> Vec<TypeTable> {
        vec![self.id.into(), self.description.into(), self.address.into()]
    }

    fn columns() -> Vec<&'static str> {
        todo!()
    }
}

impl<'a> Updatable<'a> for UpdateDevice {
    fn get_pair(self) -> Option<HashMap<&'a str, TypeTable>> {
        let mut pair = HashMap::new();

        if let Some(tmp) = self.ip {
            pair.insert("ip", tmp.into());
        }

        if let Some(tmp) = self.description {
            let data = if tmp.is_empty() { None } else { Some(tmp) };

            pair.insert("description", data.into());
        }

        if let Some(tmp) = self.network_id {
            pair.insert("network_id", tmp.into());
        }

        if let Some(tmp) = self.office_id {
            let tmp = if tmp == uuid::Uuid::nil() {
                None
            } else {
                Some(tmp)
            };
            pair.insert("office_id", tmp.into());
        }

        if let Some(tmp) = self.rack {
            let tmp = if tmp.is_empty() { None } else { Some(tmp) };
            pair.insert("rack", tmp.into());
        }

        if let Some(tmp) = self.room {
            let tmp = if tmp.is_empty() { None } else { Some(tmp) };
            pair.insert("room", tmp.into());
        }

        if let Some(status) = self.status {
            pair.insert("status", status.into());
        }

        if let Some(cred) = self.credential {
            let tmp = if cred.password.is_empty() && cred.username.is_empty() {
                None
            } else {
                Some(cred)
            };

            pair.insert("credential", tmp.into());
        }

        if !pair.is_empty() {
            Some(pair)
        } else {
            None
        }
    }
}

impl<'a> Updatable<'a> for UpdateNetwork {
    fn get_pair(self) -> Option<HashMap<&'a str, TypeTable>> {
        let mut pair = HashMap::new();

        if let Some(tmp) = self.description {
            let tmp = if tmp.is_empty() { None } else { Some(tmp) };
            pair.insert("description", tmp.into());
        }

        if let Some(tmp) = self.network {
            pair.insert("network", tmp.into());
        }

        if !pair.is_empty() {
            Some(pair)
        } else {
            None
        }
    }
}

impl<'a> Updatable<'a> for office::UpdateOffice {
    fn get_pair(self) -> Option<HashMap<&'a str, TypeTable>> {
        let mut resp = HashMap::new();
        if let Some(tmp) = self.address {
            resp.insert("address", tmp.into());
        }

        if let Some(tmp) = self.description {
            resp.insert("description", tmp.into());
        }

        Some(resp)
    }
}

#[derive(Debug)]
pub enum TypeTable {
    String(String),
    OptionUuid(Option<Uuid>),
    Uuid(Uuid),
    OptionString(Option<String>),
    Status(device::Status),
    Int32(i32),
    Role(user::Role),
    Float64(f64),
    OptionVlan(Option<i32>),
    Bytes(Option<Vec<u8>>),
}


impl From<u128> for TypeTable
{
    fn from(value: u128) -> Self {
        Self::Bytes(bincode::serialize(&value).ok())
    }
}

impl From<Option<Credential>> for TypeTable {
    fn from(value: Option<Credential>) -> Self {
        Self::Bytes(value.map(|x| bincode::serialize(&x).unwrap()))
    }
}

impl From<Option<Vlan>> for TypeTable {
    fn from(value: Option<Vlan>) -> Self {
        Self::OptionVlan(value.map(|vlan| *vlan as i32))
    }
}

impl From<Uuid> for TypeTable {
    fn from(value: Uuid) -> Self {
        TypeTable::Uuid(value)
    }
}

impl From<user::Role> for TypeTable {
    fn from(value: user::Role) -> Self {
        Self::Role(value)
    }
}

impl From<Option<Uuid>> for TypeTable {
    fn from(value: Option<Uuid>) -> Self {
        Self::OptionUuid(value)
    }
}

impl From<IpAddr> for TypeTable {
    fn from(value: IpAddr) -> Self {
        Self::String(value.to_string())
    }
}

impl From<IpNet> for TypeTable {
    fn from(value: IpNet) -> Self {
        Self::String(value.to_string())
    }
}

impl From<u8> for TypeTable {
    fn from(value: u8) -> Self {
        Self::Int32(value as i32)
    }
}

impl From<u16> for TypeTable {
    fn from(value: u16) -> Self {
        Self::Int32(value as i32)
    }
}

impl From<u32> for TypeTable {
    fn from(value: u32) -> Self {
        Self::Int32(value as i32)
    }
}

impl From<i8> for TypeTable {
    fn from(value: i8) -> Self {
        Self::Int32(value as i32)
    }
}

impl From<i16> for TypeTable {
    fn from(value: i16) -> Self {
        Self::Int32(value as i32)
    }
}

impl From<i32> for TypeTable {
    fn from(value: i32) -> Self {
        Self::Int32(value)
    }
}

impl From<String> for TypeTable {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<Option<String>> for TypeTable {
    fn from(value: Option<String>) -> Self {
        Self::OptionString(value)
    }
}

impl From<device::Status> for TypeTable {
    fn from(value: device::Status) -> Self {
        Self::Status(value)
    }
}

impl From<f32> for TypeTable {
    fn from(value: f32) -> Self {
        Self::Float64(value as f64)
    }
}
