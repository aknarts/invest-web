mod investments;
mod roles;
mod users;

use investments::Investments;
use roles::Roles;
use users::Users;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
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

pub fn switch_admin(route: AdminRoute) -> Html {
    html! {
        <div class="grid flex-fill">
            <div>
                <h1 class="title is-1">{ "Admin" }</h1>
                <ul class="nav nav-tabs">
                  <li class="nav-item">
                    <Link<AdminRoute> classes={classes!("nav-link", is_active(&route, vec![AdminRoute::Default, AdminRoute::Users]))} to={AdminRoute::Users}>{ "Users" }</Link<AdminRoute>>
                  </li>
                  <li class="nav-item">
                    <Link<AdminRoute> classes={classes!("nav-link", is_active(&route, vec![AdminRoute::Roles]))} to={AdminRoute::Roles}>{ "Roles" }</Link<AdminRoute>>
                  </li>
                  <li class="nav-item">
                    <Link<AdminRoute> classes={classes!("nav-link", is_active(&route, vec![AdminRoute::Investments]))} to={AdminRoute::Investments}>{ "Investments" }</Link<AdminRoute>>
                  </li>
                </ul>
                {route_match(&route)}
            </div>
        </div>
    }
}

fn is_active(route: &AdminRoute, desired: Vec<AdminRoute>) -> Option<String> {
    if desired.contains(route) {
        Some("active".to_string())
    } else {
        None
    }
}

fn route_match(route: &AdminRoute) -> Html {
    match route {
        AdminRoute::Default | AdminRoute::Users => html! {<Users />},
        AdminRoute::Roles => html! {<Roles />},
        AdminRoute::Investments => html! {<Investments />},
    }
}
