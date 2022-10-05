use crate::app::Route;
use crate::services::requests::set_token;
use crate::types::auth::{RegisterResponse, UserInfo};
use std::fmt;
use std::ops::Deref;
use yew::prelude::*;
use yew_router::prelude::{use_navigator, Navigator};

/// State handle for the [`use_user_context`] hook.
pub struct Handle {
    inner: UseStateHandle<UserInfo>,
    history: Navigator,
}

impl Handle {
    pub fn login(&self, value: UserInfo) {
        // Set global token after logged in
        set_token(Some(value.token.clone()));
        self.inner.set(value);
        // Redirect to home page
        self.history.push(&Route::Home);
    }

    pub fn register(&self, value: RegisterResponse) {
        // Set global token after logged in
        match value.data {
            None => {}
            Some(data) => {
                set_token(Some(data.token.clone()));
                self.inner.set(data);
                // Redirect to home page
                self.history.push(&Route::Home);
            }
        };
    }

    pub fn logout(&self) {
        // Clear global token after logged out
        self.inner.set(UserInfo::default());
        // Redirect to home page
        self.history.push(&Route::Home);
    }

    pub fn validate_email(&self, valid: bool) {
        let mut ctx = (*self.inner).clone();
        ctx.email_valid = valid;
        self.inner.set(ctx);
    }
}

impl Deref for Handle {
    type Target = UserInfo;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Clone for Handle {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            history: self.history.clone(),
        }
    }
}

impl PartialEq for Handle {
    fn eq(&self, other: &Self) -> bool {
        *self.inner == *other.inner
    }
}

impl fmt::Debug for Handle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Handle")
            .field("value", &format!("{:?}", *self.inner))
            .finish()
    }
}

#[hook]
/// This hook is used to manage user context.
pub fn use_user_context() -> Handle {
    let inner = use_context::<UseStateHandle<UserInfo>>().unwrap();
    let history = use_navigator().unwrap();

    Handle { inner, history }
}
