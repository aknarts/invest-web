use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! (
        <footer class="navbar sticky-bottom bg-light border-secondary border-top">
            <div class="container-fluid">
                <span class="navbar-text mx-auto">
                { "Powered by " }
                <a href="https://yew.rs">{ "Yew" }</a>
                </span>
            </div>
        </footer>
    )
}
