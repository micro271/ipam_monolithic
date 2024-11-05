use super::*;

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    pub id: uuid::Uuid,
    pub username: String,
    pub password: String,
    pub role: Role,
}

#[derive(Deserialize, Serialize, Debug)] 
struct UpdateUser{
    username: Option<String>,
    password: Option<String>,
    role: Option<Role>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, sqlx::Type)]
pub enum Role {
    Admin,
    Guest,
    Operator,
}