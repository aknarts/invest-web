use crate::app::Route;
use crate::error::Error;
use crate::hooks::use_user_context;
use crate::types::auth::ApiResult;
use std::collections::HashMap;
use yew::prelude::*;
use yew_hooks::{use_async, UseAsyncHandle};
use yew_router::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    let user_ctx = use_user_context();
    let active = use_state(|| false);
    let resent = use_state(|| true);
    let dropdown = use_state(|| false);
    let location = use_location().unwrap();
    let route = Route::from_path(location.path(), &HashMap::new());

    let active_class = if *active {
        (Some("show"), None)
    } else {
        (None, Some("collapsed"))
    };
    let activated = *active;

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
            <nav class="navbar navbar-expand-lg sticky-top shadow bg-light" role="navigation" aria-label="main navigation">
                <div class="container-fluid">
                    <Link<Route> to={Route::Home} classes="navbar-brand fs-2">
                        { "Invest Web" }
                    </Link<Route>>
                    <button class={classes!("navbar-toggler", active_class.1)} type="button" {onclick} aria-controls="navbarSupportedContent" aria-expanded={(!activated).to_string()} aria-label="Toggle navigation">
                      <span class="navbar-toggler-icon"></span>
                    </button>
                    <div class={classes!("collapse","navbar-collapse", active_class.0)} id="navbarSupportedContent">
                    {
                        if user_ctx.is_authenticated() {
                            logged_in_view(&user_ctx, logout, dropdown, &route)
                        } else {
                            logged_out_view(&route)
                        }
                    }
                    </div>
                </div>
            </nav>
            if !user_ctx.email_valid && user_ctx.is_authenticated() && show_resend_notification {
                <div class="alert alert-warning">
                    { "Email not verified please verify it. " }
                    <a onclick={resend}>
                        { "Resend verification email." }
                    </a>
                </div>
            }
        </>
    }
}

fn logged_out_view(route: &Option<Route>) -> Html {
    html! {
        <>
            <ul class="navbar-nav me-auto mb-2 mb-lg-0">
                <li class="nav-item">
                    <Link<Route> classes={classes!("nav-link" , is_active(route, &[Route::Home]))} to={Route::Home}>
                        { "Home" }
                    </Link<Route>>
                </li>
            </ul>
            <ul class="navbar-nav mb-2 mb-lg-0">
                <li class="nav-item">
                    <div class="btn-toolbar" role="toolbar" aria-label="Login and registration buttons">
                        <div class="btn-group me-2" role="group" aria-label="Login button group">
                            <Link<Route> classes={classes!("btn", "btn-success")} to={Route::Login}>
                                { "Login" }
                            </Link<Route>>
                        </div>
                        <div class="btn-group me-2" role="group" aria-label="Registration button group">
                            <Link<Route> classes={classes!("btn", "btn-primary")} to={Route::Register}>
                                { "Register" }
                            </Link<Route>>
                        </div>
                    </div>
                </li>
            </ul>
        </>
    }
}

fn logged_in_view(
    user_info: &crate::hooks::Handle,
    logout: UseAsyncHandle<ApiResult, Error>,
    dropdown: UseStateHandle<bool>,
    route: &Option<Route>,
) -> Html {
    let user_ctx = user_info.clone();
    let onclick = {
        Callback::from(move |_| {
            logout.run();
            // Logout current user
            user_ctx.logout();
        })
    };

    let show = if *dropdown { Some("show") } else { None };
    let dropped = *dropdown;

    let drop = {
        Callback::from(move |_| {
            dropdown.set(!*dropdown);
        })
    };

    html! {
        <>
            <ul class="navbar-nav me-auto mb-2 mb-lg-0">
                <li class="nav-item">
                    <Link<Route> classes={classes!("nav-link", is_active(route, &[Route::Overview]))} to={Route::Overview}>
                        { "Overview" }
                    </Link<Route>>
                </li>
                if user_info.check_permission("can_invest") {
                    <li class="nav-item">
                        <Link<Route> classes={classes!("nav-link", is_active(route, &[Route::Invest]))} to={Route::Invest}>
                        { "Invest" }
                        </Link<Route>>
                    </li>
                }
                if user_info.check_permission("can_invest") {
                    <li class="nav-item">
                        <Link<Route> classes={classes!("nav-link", is_active(route, &[Route::Portfolio]))} to={Route::Portfolio}>
                        { "Portfolio" }
                        </Link<Route>>
                    </li>
                }
            </ul>
            <ul class="navbar-nav mb-2 mb-lg-0">
                if user_info.check_permission("admin") {
                    <li class="nav-item">
                        <Link<Route> classes={classes!("nav-link", is_active(route, &[Route::AdminRoot]))} to={Route::AdminRoot}>
                        { "Admin" }
                        </Link<Route>>
                    </li>
                }
                <li class="nav-item dropdown">
                    <span class={classes!("nav-link", "dropdown-toggle", show)} role="button" onclick={drop} aria-expanded={
                            if dropped {
                                {"true"}
                            } else {
                                {"false"}
                            }
                        }>
                        { &user_info.username }
                    </span>
                    <ul class={classes!("dropdown-menu", "dropdown-menu-end", show)} style="position: absolute; inset: 0px 0px auto auto; margin: 0px; transform: translate(0px, 40px);">
                        <li>
                            <span class="dropdown-item-text">{ &user_info.username }</span>
                        </li>
                        <li>
                            <span class="dropdown-item-text text-muted">{ &user_info.email }</span>
                        </li>
                        <li><hr class="dropdown-divider" /></li>
                        <li>
                            <Link<Route> classes={classes!("dropdown-item")} to={Route::Profile}>
                            { "Profile" }
                            </Link<Route>>
                        </li>
                    </ul>
                </li>
                <li class="nav-item">
                    <a class="nav-link" {onclick}>
                        <i class="fa-solid fa-right-from-bracket"></i>
                    </a>
                </li>
            </ul>
        </>
    }
}

fn is_active(route: &Option<Route>, desired: &[Route]) -> Option<String> {
    route.as_ref().and_then(|r| {
        if desired.contains(r) {
            Some("active".to_string())
        } else {
            None
        }
    })
}
