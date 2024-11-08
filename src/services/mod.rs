use crate::models::{user::*, utils::*};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub id: uuid::Uuid,
    pub role: Role,
}

impl libipam::authentication::Claim for Claims {}

impl From<User> for Claims {
    fn from(value: User) -> Self {
        Self {
            exp: (time::OffsetDateTime::now_utc() + time::Duration::hours(6)).unix_timestamp()
                as usize,
            id: value.id,
            role: value.role,
        }
    }
}

impl Table for User {
    fn columns() -> Vec<&'static str> {
        vec!["id", "username", "password", "role"]
    }
    fn name() -> String {
        String::from("USERS")
    }

    fn query_insert() -> String {
        format!(
            "INSERT INTO {} (id, username, password, role) VALUES ($1, $2, $3, $4)",
            User::name()
        )
    }

    fn get_fields(self) -> Vec<TypeTable> {
        vec![
            self.id.into(),
            self.username.into(),
            self.password.into(),
            self.role.into(),
        ]
    }
}
