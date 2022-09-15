use super::requests::{request_get, request_post, request_put};
use crate::error::Error;
use crate::types::auth::*;

/// Get current user info
pub async fn current() -> Result<UserInfo, Error> {
    request_get::<UserInfo>("users".to_string()).await
}

/// Login a user
pub async fn login(login_info: LoginInfo) -> Result<UserInfo, Error> {
    request_put::<LoginInfo, UserInfo>("users".to_string(), login_info).await
}

/// Register a new user
pub async fn register(register_info: RegisterInfo) -> Result<RegisterResponse, Error> {
    request_post::<RegisterInfo, RegisterResponse>("users".to_string(), register_info).await
}
