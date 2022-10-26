use yew::prelude::*;

#[function_component(Investments)]
pub fn investments() -> Html {
    html! {
        <section class="grid flex-fill border-end border-start border-bottom">
            <div>
                <h1>{ "Investments" }</h1>
            </div>
        </section>
    }
}
