use yew::prelude::*;
use yew::{html, Html};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
    pub remove: Callback<String>,
}

#[function_component(Tag)]
pub fn tag(props: &Props) -> Html {
    let name = props.name.clone();
    let cb = props.remove.clone();
    let remove_tag = {
        Callback::from(move |_| {
            cb.emit(name.clone());
        })
    };
    html!(<span class="badge rounded-pill bg-secondary">
            { props.name.clone() }
                <button onclick={remove_tag} class="btn btn-sm m-0 py-0 px-1">{"x"}</button>
        </span>)
}