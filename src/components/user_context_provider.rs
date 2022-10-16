//! User context provider.

use crate::error::Error;
use crate::services::{auth::current, requests::get_token, requests::set_token};
use crate::types::auth::UserInfo;
use log::warn;
use yew::prelude::*;
use yew_hooks::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
}

/// User context provider.
#[function_component(UserContextProvider)]
pub fn user_context_provider(props: &Props) -> Html {
    let user_ctx = crate::hooks::use_refresh_user_context();

    html! {
        <ContextProvider<UseStateHandle<UserInfo>> context={user_ctx}>
            { for props.children.iter() }
        </ContextProvider<UseStateHandle<UserInfo>>>
    }
}
