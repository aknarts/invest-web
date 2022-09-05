use crate::components::user_context_provider::UserContextProvider;
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
                    { self.view_nav(ctx.link()) }

                    <main>
                        <Switch<Route> render={Switch::render(switch)} />
                    </main>
                    <footer class="footer">
                        <div class="content has-text-centered">
                            { "Powered by " }
                            <a href="https://yew.rs">{ "Yew" }</a>
                        </div>
                    </footer>
                </BrowserRouter>
            </UserContextProvider>
        }
    }
}

impl App {
    fn view_nav(&self, link: &Scope<Self>) -> Html {
        let Self { navbar_active, .. } = *self;

        let active_class = if !navbar_active { "is-active" } else { "" };

        html! {
            <nav class="navbar is-primary" role="navigation" aria-label="main navigation">
                <div class="navbar-brand">
                    <h1 class="navbar-item is-size-3">{ "Invest Web" }</h1>

                    <button class={classes!("navbar-burger", "burger", active_class)}
                        aria-label="menu" aria-expanded="false"
                        onclick={link.callback(|_| Msg::ToggleNavbar)}
                    >
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                    </button>
                </div>
                <div class={classes!("navbar-menu", active_class)}>
                    <div class="navbar-start">
                        <Link<Route> classes={classes!("navbar-item")} to={Route::Home}>
                            { "Home" }
                        </Link<Route>>
                    </div>
                <div class="navbar-end">
                        <Link<Route> classes={classes!("navbar-item")} to={Route::Login}>
                            { "Login" }
                        </Link<Route>>
                        <Link<Route> classes={classes!("navbar-item")} to={Route::Register}>
                            { "Register" }
                        </Link<Route>>
                    </div>
                </div>
            </nav>
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
