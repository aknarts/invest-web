use yew::prelude::*;

#[function_component(Overview)]
pub fn overview() -> Html {
    html! {
        <div class="grid flex-fill">
            <div>
                <h1 class="title is-1">{ "Overview" }</h1>
            </div>
        </div>
    }
}
