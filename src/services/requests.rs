use crate::error::Error;
use crate::types::auth::ApiResult;
use crate::types::ErrorInfo;
use gloo::storage::{LocalStorage, Storage};
use lazy_static::lazy_static;
use log::debug;
use parking_lot::RwLock;
use serde::{de::DeserializeOwned, Serialize};

const API_ROOT: &str = "http://investown2.test:8081/";
const TOKEN_KEY: &str = "invest.token";

lazy_static! {
    /// Jwt token read from local storage.
    pub static ref TOKEN: RwLock<Option<String>> = {
        if let Ok(token) = LocalStorage::get(TOKEN_KEY) {
            RwLock::new(Some(token))
        } else {
            RwLock::new(None)
        }
    };
}

/// Set jwt token to local storage.
pub fn set_token(token: Option<String>) {
    token.clone().map_or_else(
        || {
            LocalStorage::delete(TOKEN_KEY);
        },
        |t| {
            LocalStorage::set(TOKEN_KEY, t).expect("failed to set");
        },
    );
    let mut token_lock = TOKEN.write();
    *token_lock = token;
}

/// Get jwt token from lazy static.
pub fn get_token() -> Option<String> {
    let token_lock = TOKEN.read();
    token_lock.clone()
}

/// build all kinds of http request: post/get/delete etc.
pub async fn request<B, T>(method: reqwest::Method, url: String, body: B) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
    B: Serialize + std::fmt::Debug,
{
    let allow_body = method == reqwest::Method::POST || method == reqwest::Method::PUT;
    let url = format!("{}{}", API_ROOT, url);
    let mut builder = reqwest::Client::new().request(method, &url);
    if let Some(token) = get_token() {
        builder = builder.bearer_auth(token);
    }

    debug!("url: {}", url);

    if allow_body {
        builder = builder.json(&body);
    }

    let response = builder.send().await;

    if let Ok(data) = response {
        if data.status().is_success() {
            let data: Result<T, _> = data.json::<T>().await;
            if let Ok(data) = data {
                log::debug!("Response: {:?}", data);
                Ok(data)
            } else {
                Err(Error::DeserializeError)
            }
        } else {
            match data.status().as_u16() {
                400 => Err(Error::BadRequest),
                401 => {
                    let data: Result<ApiResult, _> = data.json::<ApiResult>().await;
                    match data {
                        Ok(d) => Err(Error::Unauthorized(d.result)),
                        Err(e) => {
                            debug!("Failed to deserialize response: {e}");
                            Err(Error::DeserializeError)
                        }
                    }
                }
                403 => Err(Error::Forbidden),
                404 => Err(Error::NotFound),
                409 => {
                    let data: Result<ApiResult, _> = data.json::<ApiResult>().await;
                    match data {
                        Ok(d) => Err(Error::Conflict(d.result)),
                        Err(e) => {
                            debug!("Failed to deserialize response: {e}");
                            Err(Error::DeserializeError)
                        }
                    }
                }
                500 => Err(Error::InternalServerError),
                422 => {
                    let data: Result<ErrorInfo, _> = data.json::<ErrorInfo>().await;
                    if let Ok(data) = data {
                        Err(Error::UnprocessableEntity(data))
                    } else {
                        Err(Error::DeserializeError)
                    }
                }
                _ => Err(Error::RequestError),
            }
        }
    } else {
        Err(Error::RequestError)
    }
}

/// Delete request
#[allow(dead_code)]
pub async fn request_delete<T>(url: String) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
{
    request(reqwest::Method::DELETE, url, ()).await
}

/// Get request
pub async fn request_get<T>(url: String) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
{
    request(reqwest::Method::GET, url, ()).await
}

/// Post request with a body
pub async fn request_post<B, T>(url: String, body: B) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
    B: Serialize + std::fmt::Debug,
{
    request(reqwest::Method::POST, url, body).await
}

/// Put request with a body
pub async fn request_put<B, T>(url: String, body: B) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
    B: Serialize + std::fmt::Debug,
{
    request(reqwest::Method::PUT, url, body).await
}

/// Patch request with a body
pub async fn request_patch<B, T>(url: String, body: B) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
    B: Serialize + std::fmt::Debug,
{
    request(reqwest::Method::PATCH, url, body).await
}
