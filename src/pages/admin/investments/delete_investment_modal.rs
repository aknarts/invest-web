use crate::services::admin::{Investment, RoleId};
use yew::prelude::*;
use yew_hooks::{use_async, UseCounterHandle};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub investment: Option<Investment>,
    pub close: Callback<MouseEvent>,
    pub counter: UseCounterHandle,
}

#[function_component(DeleteInvestment)]
pub fn delete_role(props: &Props) -> Html {
    let investment = props
        .investment
        .as_ref()
        .map_or_else(Investment::default, Clone::clone);

    let investment_delete = {
        let investment = investment.clone();
        use_async(
            async move { crate::services::admin::delete_role(RoleId { id: investment.id }).await },
        )
    };

    let onsubmit = {
        let investment_delete = investment_delete.clone();
        let close = props.close.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            investment_delete.run();
            close.emit(MouseEvent::new("mousedown").unwrap());
        })
    };

    {
        let counter = props.counter.clone();
        use_effect_with_deps(
            move |investment_delete| {
                if investment_delete.data.is_some() {
                    counter.increase();
                }
                || ()
            },
            investment_delete,
        );
    };

    html!(
        <>
            <div class="modal-body">
                { format!("Do you really want to remove investment \"{}\"", investment.name) }
            </div>
            <div class="modal-footer">
                <form {onsubmit} class="btn-group">
                    <button type="button" onclick={&props.close} class="btn btn-secondary">{"Cancel"}</button>
                    <button type="submit" class="btn btn-danger">{"Confirm"}</button>
                </form>
            </div>
        </>
    )
}
