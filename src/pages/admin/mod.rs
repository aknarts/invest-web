mod investments;
mod roles;
mod users;
use crate::app::Route;
use crate::hooks::use_user_context;
use investments::Investments;
use roles::Roles;
use users::Users;
use yew::prelude::*;
use yew_router::prelude::*;

#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Routable, Eq, PartialEq)]
pub enum AdminRoute {
    #[at("/admin")]
    Default,
    #[at("/admin/users")]
    Users,
    #[at("/admin/roles")]
    Roles,
    #[at("/admin/investments")]
    Investments,
}

#[allow(clippy::module_name_repetitions)]
#[allow(clippy::needless_pass_by_value)]
pub fn switch_admin(route: AdminRoute) -> Html {
    html! {
        <Admin route={route} />
    }
}

#[derive(Properties, Clone, Eq, PartialEq)]
pub struct Props {
    pub route: AdminRoute,
}

#[function_component(Admin)]
pub fn admin(props: &Props) -> Html {
    let route = &props.route;
    let user_ctx = use_user_context();
    html! {
        <div class="grid flex-fill">
            <div>
                <h1 class="title is-1">{ "Admin" }</h1>
                <ul class="nav nav-tabs">
                    if user_ctx.check_permission("list_users") {
                        <li class="nav-item">
                            <Link<AdminRoute> classes={classes!("nav-link", is_active(route, &[AdminRoute::Default, AdminRoute::Users]))} to={AdminRoute::Users}>{ "Users" }</Link<AdminRoute>>
                        </li>
                    }
                    if user_ctx.check_permission("list_roles") {
                        <li class="nav-item">
                            <Link<AdminRoute> classes={classes!("nav-link", is_active(route, &[AdminRoute::Roles]))} to={AdminRoute::Roles}>{ "Roles" }</Link<AdminRoute>>
                        </li>
                    }
                    if user_ctx.check_permission("list_investments") {
                        <li class="nav-item">
                            <Link<AdminRoute> classes={classes!("nav-link", is_active(route, &[AdminRoute::Investments]))} to={AdminRoute::Investments}>{ "Investments" }</Link<AdminRoute>>
                        </li>
                    }
                </ul>
                {route_match(route, &user_ctx)}
            </div>
        </div>
    }
}

fn is_active(route: &AdminRoute, desired: &[AdminRoute]) -> Option<String> {
    if desired.contains(route) {
        Some("active".to_string())
    } else {
        None
    }
}

fn route_match(route: &AdminRoute, user_info: &crate::hooks::Handle) -> Html {
    match route {
        AdminRoute::Default => {
            if user_info.check_permission("list_users") {
                html! {<Users />}
            } else if user_info.check_permission("list_roles") {
                html! {<Roles />}
            } else if user_info.check_permission("list_investments") {
                html! {<Investments />}
            } else {
                user_info.navigate_to(&Route::Home);
                html!("Unauthorized")
            }
        }
        AdminRoute::Users => html! {<Users />},
        AdminRoute::Roles => html! {<Roles />},
        AdminRoute::Investments => html! {<Investments />},
    }
}
