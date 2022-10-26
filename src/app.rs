use crate::components::user_context_provider::UserContextProvider;
use crate::pages::admin::AdminRoute;
use crate::pages::confirm_email::ConfirmEmail;
use crate::pages::footer::Footer;
use crate::pages::header::Header;
use crate::pages::home::Home;
use crate::pages::invest::Invest;
use crate::pages::login::Login;
use crate::pages::overview::Overview;
use crate::pages::page_not_found::PageNotFound;
use crate::pages::portfolio::Portfolio;
use crate::pages::profile::Profile;
use crate::pages::register::Register;
use log::debug;
use yew::html::Html;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
    #[at("/confirm_email")]
    ConfirmEmail,
    #[at("/admin")]
    AdminRoot,
    #[at("/admin/*")]
    Admin,
    #[at("/overview")]
    Overview,
    #[at("/invest")]
    Invest,
    #[at("/portfolio")]
    Portfolio,
    #[at("/profile")]
    Profile,
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <UserContextProvider>
            <BrowserRouter>
                <Header />
                <div class="d-flex p-2 flex-grow-1">
                    <main class="container-sm d-flex">
                        <Switch<Route> render={switch} />
                    </main>
                </div>
                <Footer />
            </BrowserRouter>
        </UserContextProvider>
    }
}

// Allowing because this is how yew defines the function
#[allow(clippy::needless_pass_by_value)]
fn switch(routes: Route) -> Html {
    debug!("Routing to {:?}", routes);
    match routes {
        Route::Home => html! { <Home /> },
        Route::NotFound => html! { <PageNotFound /> },
        Route::Login => html! {<Login />},
        Route::Register => html! {<Register />},
        Route::ConfirmEmail => html! {<ConfirmEmail />},
        Route::Admin | Route::AdminRoot => {
            html! {<Switch<AdminRoute> render={crate::pages::admin::switch_admin} />}
        }
        Route::Overview => html! {<Overview />},
        Route::Invest => html! {<Invest />},
        Route::Portfolio => html! {<Portfolio />},
        Route::Profile => html! {<Profile />},
    }
}
