use crate::components::user_context_provider::UserContextProvider;
use crate::hooks::use_user_context;
use crate::pages::footer::Footer;
use crate::pages::header::Header;
use crate::pages::home::Home;
use crate::pages::login::Login;
use crate::pages::page_not_found::PageNotFound;
use crate::pages::register::Register;
use yew::html::Scope;
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
}

pub enum Msg {
    ToggleNavbar,
}

pub struct App {
    navbar_active: bool,
}
impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            navbar_active: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleNavbar => {
                self.navbar_active = !self.navbar_active;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <UserContextProvider>
                <BrowserRouter>
                    <Header />
                    <main class="section is-large">
                        <Switch<Route> render={Switch::render(switch)} />
                    </main>
                    <Footer />
                </BrowserRouter>
            </UserContextProvider>
        }
    }
}

fn switch(routes: &Route) -> Html {
    println!("Routing to {:?}", routes);
    match routes {
        Route::Home => html! { <Home /> },
        Route::NotFound => html! { <PageNotFound /> },
        Route::Login => html! {<Login />},
        Route::Register => html! {<Register />},
    }
}
