use crate::pages::admin::investments::modal::{InvestmentAction, InvestmentInfo};
use yew::prelude::*;
use yew::{html, Html};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
    pub callback: UseReducerDispatcher<InvestmentInfo>,
}

#[function_component(Tag)]
pub fn tag(props: &Props) -> Html {
    let name = props.name.clone();
    let remove_tag = {
        let callback = props.callback.clone();
        Callback::from(move |_| callback.dispatch(InvestmentAction::RemoveTag(name.clone())))
    };
    html!(<span class="badge rounded-pill bg-secondary align-middle">
            { props.name.clone() }
                <button onclick={remove_tag} class="btn btn-sm m-0 py-0 px-1">{"x"}</button>
        </span>)
}
