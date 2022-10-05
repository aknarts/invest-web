use crate::components::user_context_provider::UserContextProvider;
use crate::pages::confirm_email::ConfirmEmail;
use crate::pages::footer::Footer;
use crate::pages::header::Header;
use crate::pages::home::Home;
use crate::pages::login::Login;
use crate::pages::page_not_found::PageNotFound;
use crate::pages::register::Register;
use log::debug;
use yew::html::*;
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
}

pub(crate) struct App {}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <UserContextProvider>
                <BrowserRouter>
                    <Header />
                    <main class="section is-large">
                        <Switch<Route> render={switch} />
                    </main>
                    <Footer />
                </BrowserRouter>
            </UserContextProvider>
        }
    }
}

fn switch(routes: Route) -> Html {
    debug!("Routing to {:?}", routes);
    match routes {
        Route::Home => html! { <Home /> },
        Route::NotFound => html! { <PageNotFound /> },
        Route::Login => html! {<Login />},
        Route::Register => html! {<Register />},
        Route::ConfirmEmail => html! {<ConfirmEmail />},
    }
}
