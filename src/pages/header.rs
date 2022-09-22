use yew::prelude::*;
use yew_hooks::use_async;
use yew_router::prelude::*;

use crate::app::Route;
use crate::hooks::use_user_context;
use crate::types::auth::UserInfo;

#[function_component(Header)]
pub fn header() -> Html {
    let user_ctx = use_user_context();
    let active = use_state(|| true);
    let resent = use_state(|| true);

    let active_class = if *active { "" } else { "is-active" };

    let onclick = { Callback::from(move |_| active.set(!*active)) };

    let resend_email = use_async(crate::services::auth::resend());

    let show_resend_notification = *resent;

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
                    <Link<Route> to={Route::Home} classes="navbar-item is-size-3 h1">
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
                        logged_in_view(&*user_ctx, active_class.to_string())
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

fn logged_in_view(user_info: &UserInfo, active_class: String) -> Html {
    let user_ctx = use_user_context();
    let logout = use_async(crate::services::auth::logout());
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
                <Link<Route> classes={classes!("navbar-item")} to={Route::Home}>
                    { "Home" }
                </Link<Route>>
                <hr class="navbar-divider"/>
            </div>
            <div class="navbar-end">
                <div class="navbar-item has-dropdown is-hoverable">
                    <a class="navbar-link">{ &user_info.username }</a>
                    <div class="navbar-dropdown">
                        <a class="navbar-item" {onclick}>
                            { "Logout" }
                        </a>
                    </div>
                </div>
            </div>
        </div>
    }
}
