use crate::error::Error;
use crate::services::requests::request_get;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Default)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub username: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Default)]
pub struct Role {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub permissions: Option<Vec<Permission>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Default)]
pub struct Permission {
    pub id: i32,
    pub name: String,
    pub description: String,
}

pub async fn get_user_list() -> Result<Vec<User>, Error> {
    request_get::<Vec<User>>("/admin/users".to_string()).await
}

pub async fn get_role_list() -> Result<Vec<Role>, Error> {
    request_get::<Vec<Role>>("/admin/roles".to_string()).await
}

pub async fn get_permissions_list() -> Result<Vec<Permission>, Error> {
    request_get::<Vec<Permission>>("/admin/permissions".to_string()).await
}
