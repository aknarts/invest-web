use yew::prelude::*;

use crate::error::Error;

#[derive(Properties, Clone, Eq, PartialEq)]
pub struct Props {
    pub error: Option<Error>,
}

#[function_component(ListErrors)]
pub fn list_errors(props: &Props) -> Html {
    props.error.as_ref().map_or_else(
        || html! {},
        |error| {
            html! {
                html! {
                    <div class="notification is-danger is-light">
                        {
                             {error}
                        }
                    </div>
                }
            }
        },
    )
}
