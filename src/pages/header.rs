use yew::prelude::*;
use yew_hooks::{use_async, UseAsyncHandle};
use yew_router::prelude::*;

use crate::app::Route;
use crate::error::Error;
use crate::hooks::use_user_context;
use crate::types::auth::ApiResult;

#[function_component(Header)]
pub fn header() -> Html {
    let user_ctx = use_user_context();
    let active = use_state(|| true);
    let resent = use_state(|| true);

    let active_class = if *active { "" } else { "is-active" };

    let onclick = { Callback::from(move |_| active.set(!*active)) };

    let resend_email = use_async(crate::services::auth::resend());

    let show_resend_notification = *resent;

    let logout = use_async(crate::services::auth::logout());

    let resend = {
        Callback::from(move |_| {
            resent.set(!*resent);
            resend_email.run();
        })
    };

    html! {
        <>
            <nav class="navbar is-transparent is-fixed-top has-shadow" role="navigation" aria-label="main navigation">
                <div class="navbar-brand">
                    <Link<Route> to={Route::Home} classes="navbar-item is-size-3 has-text-weight-semibold">
                        { "Invest Web" }
                    </Link<Route>>
                    <button class={classes!("navbar-burger", "burger", active_class)}
                            aria-label="menu" aria-expanded="false"
                            {onclick}
                        >
                            <span aria-hidden="true"></span>
                            <span aria-hidden="true"></span>
                            <span aria-hidden="true"></span>
                    </button>
                </div>
                {
                    if user_ctx.is_authenticated() {
                        logged_in_view(&user_ctx, active_class.to_string(), logout)
                    } else {
                        logged_out_view(active_class.to_string())
                    }
                }
            </nav>
            if !user_ctx.email_valid && user_ctx.is_authenticated() && show_resend_notification {
                <div class="notification is-warning">
                    { "Email not verified please verify it. " }
                    <a onclick={resend}>
                        { "Resend verification email." }
                    </a>
                </div>
            }
        </>
    }
}

fn logged_out_view(active_class: String) -> Html {
    html! {
        <div class={classes!("navbar-menu", active_class)}>
            <div class="navbar-start">
                <Link<Route> classes={classes!("navbar-item")} to={Route::Home}>
                    { "Home" }
                </Link<Route>>
            <hr class="navbar-divider"/>
            </div>
            <div class="navbar-end">
                <div class="navbar-item">
                    <div class="field is-grouped">
                        <p class="control">
                            <Link<Route> classes={classes!("button", "is-light")} to={Route::Login}>
                                { "Login" }
                            </Link<Route>>
                        </p>
                        <p class="control">
                            <Link<Route> classes={classes!("button", "is-primary")} to={Route::Register}>
                                { "Register" }
                            </Link<Route>>
                        </p>
                    </div>
                </div>
            </div>
        </div>
    }
}

fn logged_in_view(
    user_info: &crate::hooks::Handle,
    active_class: String,
    logout: UseAsyncHandle<ApiResult, Error>,
) -> Html {
    let user_ctx = user_info.clone();
    let onclick = {
        Callback::from(move |_| {
            logout.run();
            // Logout current user
            user_ctx.logout();
        })
    };

    html! {
        <div class={classes!("navbar-menu", active_class)}>
            <div class="navbar-start">
                <Link<Route> classes={classes!("navbar-item")} to={Route::Overview}>
                    { "Overview" }
                </Link<Route>>
                if user_info.check_permission("can_invest") {
                    <Link<Route> classes={classes!("navbar-item")} to={Route::Invest}>
                    { "Invest" }
                    </Link<Route>>
                }
                if user_info.check_permission("can_invest") {
                    <Link<Route> classes={classes!("navbar-item")} to={Route::Portfolio}>
                    { "Portfolio" }
                    </Link<Route>>
                }
                <hr class="navbar-divider"/>
            </div>
            <div class="navbar-end">
                if user_info.check_permission("admin") {
                    <Link<Route> classes={classes!("navbar-item")} to={Route::Admin}>
                    { "Admin" }
                    </Link<Route>>
                }
                <div class="navbar-item has-dropdown is-hoverable">
                    <a class="navbar-link">{ &user_info.username }</a>
                    <div class="navbar-dropdown is-right">
                        <div class="navbar-item has-background-light">
                            <div class="is-spaced">
                                <p class="title is-6">{ &user_info.username }</p>
                                <p class="subtitle is-7">{ &user_info.email }</p>
                            </div>
                        </div>
                        <Link<Route> classes={classes!("navbar-item")} to={Route::Profile}>
                        { "Profile" }
                        </Link<Route>>
                    </div>
                </div>
                <a class="navbar-item" {onclick}>
                    <i class="fa-solid fa-right-from-bracket"></i>
                </a>
            </div>
        </div>
    }
}
