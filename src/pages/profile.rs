use yew::prelude::*;

#[function_component(Profile)]
pub fn profile() -> Html {
    html! (
        <div class="grid flex-fill">
            <div>
                <h1 class="title is-1">{ "Profile" }</h1>
            </div>
        </div>
    )
}
