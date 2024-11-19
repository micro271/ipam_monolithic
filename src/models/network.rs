use ipnet::IpNet;
use libipam::type_net::host_count::HostCount;
use serde::{
    de::{self, Visitor},
    Deserialize, Serialize,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateNetwork {
    pub network: Option<IpNet>,
    pub description: Option<String>,
    pub vlan: Option<Vlan>,
}

#[derive(Debug)]
pub struct UpdateNetworkCount {
    pub used: Option<HostCount>,
    pub free: Option<HostCount>,
    pub available: Option<HostCount>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Network {
    pub id: uuid::Uuid,
    pub vlan: Option<Vlan>,
    pub network: IpNet,
    pub description: Option<String>,
    pub available: HostCount,
    pub used: HostCount,
    pub free: HostCount,
}

impl std::ops::Deref for Vlan {
    type Target = u16;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for Vlan {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Clone)]
pub struct Vlan(u16);

impl Vlan {
    pub fn new(num: u16) -> Result<Self, VlanError> {
        let mut tmp = Self(0);
        tmp.set_vlan(num)?;
        Ok(tmp)
    }

    pub fn set_vlan(&mut self, num: u16) -> Result<(), VlanError> {
        if num > 4096 {
            return Err(VlanError::InvalidVlan);
        }
        **self = num;

        Ok(())
    }
}

#[derive(Debug)]
pub enum VlanError {
    InvalidVlan,
}

impl Serialize for Vlan {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u16(**self)
    }
}

impl<'de> Deserialize<'de> for Vlan {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(VlanVisitor(()))
    }
}

struct VlanVisitor(());

impl<'de> Visitor<'de> for VlanVisitor {
    type Value = Vlan;

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Vlan(v))
    }

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid VLAN ID as a u16 or a string representing a u16")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        v.parse::<u16>().map(Vlan).map_err(|_| {
            de::Error::invalid_value(
                de::Unexpected::Str(v),
                &"An string that representing a valid Vlan id",
            )
        })
    }
}
