use super::*;

#[derive(Deserialize, Serialize)]
pub struct User {
    pub id: uuid::Uuid,
    pub username: String,
    pub password: String,
    pub role: Role,
}

#[derive(Deserialize, Serialize)] 
struct UpdateUser{
    username: Option<String>,
    password: Option<String>,
    role: Option<Role>,
}

#[derive(Deserialize, Serialize, sqlx::Type, Debug, Clone, PartialEq)]
pub enum Role {
    Admin,
    Guest,
    Operator,
}