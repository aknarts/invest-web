use std::ops::Not;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::Route;
use crate::hooks::use_user_context;
use crate::types::auth::UserInfo;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="footer">
            <div class="content has-text-centered">
                { "Powered by " }
                <a href="https://yew.rs">{ "Yew" }</a>
            </div>
        </footer>
    }
}
