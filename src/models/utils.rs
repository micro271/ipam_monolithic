use super::device::*;
use super::{network::*, *};
use ipnet::IpNet;
use libipam::type_net::host_count::HostCount;
use libipam::type_net::port::Port;
use service::ServicesUpdate;
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
            "location",
            "status",
            "network_id",
            "credential",
        ]
    }

    fn name() -> String {
        String::from("devices")
    }

    fn query_insert() -> String {
        format!("INSERT INTO {} (ip, network_id, description, location, status, credential) VALUES ($1, $2, $3, $4, $5, $6)", Self::name())
    }

    fn get_fields(self) -> Vec<TypeTable> {
        vec![
            self.ip.into(),
            self.network_id.into(),
            self.description.into(),
            self.location.into(),
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
            "free",
            "description",
            "father",
        ]
    }

    fn name() -> String {
        String::from("networks")
    }

    fn query_insert() -> String {
        format!(
            "INSERT INTO {} (id, network, available, used, free, vlan, description, father) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
            Self::name()
        )
    }

    fn get_fields(self) -> Vec<TypeTable> {
        vec![
            self.id.into(),
            self.network.into(),
            self.available.into(),
            self.used.into(),
            self.free.into(),
            self.vlan.into(),
            self.description.into(),
            self.father.into(),
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
        vec!["id", "description", "address"]
    }
}

impl Table for service::Services {
    fn name() -> String {
        String::from("services")
    }

    fn query_insert() -> String {
        format!(
            "INSERT INTO {} (id, name, version) VALUES ($1, $2, $3)",
            Self::name()
        )
    }

    fn get_fields(self) -> Vec<TypeTable> {
        vec![self.id.into(), self.name.into(), self.version.into()]
    }

    fn columns() -> Vec<&'static str> {
        vec!["id", "name", "version"]
    }
}

impl Table for service::Service {
    fn name() -> String {
        String::from("services")
    }

    fn query_insert() -> String {
        format!(
            "INSERT INTO {} (port, ip, network_id, service_id, descripcion) VALUES ($1, $2, $3, $4, $5)",
            Self::name()
        )
    }

    fn get_fields(self) -> Vec<TypeTable> {
        vec![
            self.port.into(),
            self.ip.into(),
            self.netwok_id.into(),
            self.service_id.into(),
            self.description.into(),
        ]
    }

    fn columns() -> Vec<&'static str> {
        vec![
            "port",
            "ip",
            "network_id",
            "service_id",
            "description",
        ]
    }
}

impl<'a> Updatable<'a> for UpdateDevice {
    fn get_pair(self) -> Option<HashMap<&'a str, TypeTable>> {
        let mut pair = HashMap::new();

        if let Some(tmp) = self.ip {
            pair.insert("ip", tmp.into());
        }

        if let Some(tmp) = self.description {
            pair.insert("description", if tmp.is_empty() { 
                None 
            } else { 
                Some(tmp) 
            }.into());
        }

        if let Some(tmp) = self.network_id {
            pair.insert("network_id", tmp.into());
        }

        if let Some(tmp) = self.location {
            let tmp = if tmp.is_empty() { None } else { Some(tmp) };
            pair.insert("rack", tmp.into());
        }

        if let Some(cred) = self.credential {
            pair.insert("credential", if cred.password.is_empty() && cred.username.is_empty() {
                None
            } else {
                Some(cred)
            }.into());
        }

        if !pair.is_empty() {
            Some(pair)
        } else {
            None
        }
    }
}

impl<'a> Updatable<'a> for Status {
    fn get_pair(self) -> Option<HashMap<&'a str, TypeTable>> {
        Some(HashMap::from([("status", self.into())]))
    }
}

impl<'a> Updatable<'a> for UpdateNetwork {
    fn get_pair(self) -> Option<HashMap<&'a str, TypeTable>> {
        let mut pair = HashMap::new();

        if let Some(tmp) = self.description {
            pair.insert("description", if tmp.is_empty() { 
                None 
            } else { 
                Some(tmp) 
            }.into());
        }

        if let Some(tmp) = self.network {
            pair.insert("network", tmp.into());
        }

        if let Some(vlan) = self.vlan {
            pair.insert("vlan", Some(vlan).into());
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

impl<'a> Updatable<'a> for network::UpdateNetworkCount {
    fn get_pair(self) -> Option<HashMap<&'a str, TypeTable>> {
        let mut resp = HashMap::new();
        if let Some(tmp) = self.used {
            resp.insert("used", tmp.into());
        }

        if let Some(tmp) = self.free {
            resp.insert("free", tmp.into());
        }

        if let Some(tmp) = self.available {
            resp.insert("available", tmp.into());
        }

        Some(resp)
    }
}

impl<'a> Updatable<'a> for service::ServiceUpdate {
    fn get_pair(self) -> Option<HashMap<&'a str, TypeTable>> {
        let mut resp: HashMap<&str, TypeTable> = HashMap::new();

        if let Some(tmp) = self.port {
            resp.insert("port", tmp.into());
        }
        if let Some(tmp) = self.description {
            resp.insert(
                "description",
                if tmp.is_empty() { None } else { Some(tmp) }.into(),
            );
        }
        if let Some(tmp) = self.ip {
            resp.insert("ip", tmp.into());
        }
        if let Some(tmp) = self.netwok_id {
            resp.insert("network_id", tmp.into());
        }
        if let Some(tmp) = self.service_id {
            resp.insert(
                "service_id",
                if tmp.is_nil() { None } else { Some(tmp) }.into(),
            );
        }

        Some(resp)
    }
}

impl<'a> Updatable<'a> for ServicesUpdate {
    fn get_pair(self) -> Option<HashMap<&'a str, TypeTable>> {
        let mut resp = HashMap::new();
        if let Some(e) = self.name {
            resp.insert("name", if e.is_empty() { None } else { Some(e) }.into());
        }

        if let Some(e) = self.version {
            resp.insert("version", if e.is_empty() { None } else { Some(e) }.into());
        }

        Some(resp)
    }
}

#[derive(Debug, PartialEq)]
pub enum TypeTable {
    String(String),
    OptionUuid(Option<Uuid>),
    Uuid(Uuid),
    OptionString(Option<String>),
    Status(device::Status),
    U32(u32),
    Role(user::Role),
    OptionU16(Option<u16>),
    BytesOption(Option<Vec<u8>>),
    U16(u16),
    Null,
}

impl From<Port> for TypeTable {
    fn from(value: Port) -> Self {
        Self::U16(*value)
    }
}

impl From<Option<Credential>> for TypeTable {
    fn from(value: Option<Credential>) -> Self {
        Self::BytesOption(value.map(|x| bincode::serialize(&x).unwrap()))
    }
}

impl From<Option<Vlan>> for TypeTable {
    fn from(value: Option<Vlan>) -> Self {
        Self::OptionU16(value.map(|vlan| *vlan))
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

impl From<HostCount> for TypeTable {
    fn from(value: HostCount) -> Self {
        Self::U32(*value)
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
