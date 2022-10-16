use yew::prelude::*;

#[function_component(Overview)]
pub fn overview() -> Html {
    html! {
        <div class="tile is-ancestor is-vertical">
            <div class="tile is-child hero">
                <div class="hero-body container pb-0">
                    <h1 class="title is-1">{ "Overview" }</h1>
                </div>
            </div>
        </div>
    }
}
