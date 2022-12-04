use crate::error::Error;
use crate::services::requests::{request_get, request_post, request_put};
use crate::types::auth::ApiResult;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

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

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct RoleInfo {
    pub id: Option<i32>,
    pub name: String,
    pub description: String,
    pub permissions: HashSet<String>,
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

pub async fn create_role(new_role: RoleInfo) -> Result<ApiResult, Error> {
    request_post::<RoleInfo, ApiResult>("/admin/roles".to_string(), new_role).await
}

pub async fn edit_role(role: RoleInfo) -> Result<ApiResult, Error> {
    request_put::<RoleInfo, ApiResult>("/admin/roles".to_string(), role).await
}
