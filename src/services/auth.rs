use super::requests::{request_get, request_post, request_put};
use crate::error::Error;
use crate::services::requests::{request_patch, set_token};
use crate::types::auth::{
    ApiResult, EmailConfirmationResult, EmailResendInfo, LoginInfo, RegisterInfo, RegisterResponse,
    UserInfo,
};

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

/// Get current user info
pub async fn logout() -> Result<ApiResult, Error> {
    let result = request_patch::<(), ApiResult>("users".to_string(), ()).await;
    set_token(None);
    result
}

/// Get current user info
pub async fn confirm_email(code: &str) -> Result<EmailConfirmationResult, Error> {
    request_get::<EmailConfirmationResult>(format!("/users/email?code={code}")).await
}

/// Get current user info
pub async fn resend(user_id: i64, email: String) -> Result<ApiResult, Error> {
    request_patch::<EmailResendInfo, ApiResult>(
        "/users/email".to_string(),
        EmailResendInfo { user_id, email },
    )
    .await
}
