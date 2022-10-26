use yew::prelude::*;

#[function_component(Users)]
pub fn users() -> Html {
    html! {
        <section class="grid flex-fill border-end border-start border-bottom">
            <div>
                <h1>{ "Users" }</h1>
            </div>
        </section>
    }
}
