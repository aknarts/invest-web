use crate::app::Route;
use crate::error::Error;
use crate::services::auth::current;
use crate::services::requests::{get_token, set_token};
use crate::types::auth::{RegisterResponse, UserInfo};
use log::warn;
use std::fmt;
use std::ops::Deref;
use yew::prelude::*;
use yew_hooks::{use_async, use_mount};
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
        self.history.push(&Route::Overview);
    }

    pub fn check_permission(&self, permission: &str) -> bool {
        // Check foR Admin
        if self.permissions.contains(&"*".to_string()) {
            return true;
        };
        if self.permissions.contains(&permission.to_string()) {
            return true;
        };
        false
    }

    pub fn navigate_to(&self, route: &Route) {
        self.history.push(route);
    }

    pub fn register(&self, value: RegisterResponse) {
        // Set global token after logged in
        if let Some(data) = value.data {
            set_token(Some(data.token.clone()));
            self.inner.set(data);
            // Redirect to home page
            self.history.push(&Route::Profile);
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

#[hook]
/// This hook is used to manage user context.
pub fn use_refresh_user_context() -> UseStateHandle<UserInfo> {
    #[allow(clippy::or_fun_call)]
    let user_ctx =
        use_context::<UseStateHandle<UserInfo>>().unwrap_or(use_state(UserInfo::default));
    let current_user = use_async(async move { current().await });

    {
        let current_user = current_user.clone();
        use_mount(move || {
            if get_token().is_some() {
                current_user.run();
            }
        });
    }

    {
        let user_ctx = user_ctx.clone();
        use_effect_with_deps(
            move |current_user| {
                if let Some(user_info) = &current_user.data {
                    user_ctx.set(user_info.clone());
                }

                if let Some(error) = &current_user.error {
                    if let Error::Unauthorized(s) | Error::Forbidden(s) = error {
                        warn!("Unauthorized {s}");
                        set_token(None);
                    }
                }
                || ()
            },
            current_user,
        );
    };
    user_ctx
}
