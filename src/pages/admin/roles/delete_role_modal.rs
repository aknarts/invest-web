use yew::prelude::*;
use yew_hooks::{use_async, UseCounterHandle};
use crate::services::admin::{Role, RoleId};

#[derive(Properties, PartialEq)]
pub struct DeleteRoleProps {
    pub role: Option<Role>,
    pub close: Callback<MouseEvent>,
    pub counter: UseCounterHandle,
}

#[function_component(DeleteRole)]
pub fn delete_role(props: &DeleteRoleProps) -> Html {
    let role = props.role.as_ref().map_or(
        Role {
            id: -1,
            name: String::new(),
            description: String::new(),
            permissions: None,
        },
        Clone::clone,
    );

    let role_delete = {
        let role = role.clone();
        use_async(async move { crate::services::admin::delete_role(RoleId { id: role.id }).await })
    };

    let onsubmit = {
        let role_delete = role_delete.clone();
        let close = props.close.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            role_delete.run();
            close.emit(MouseEvent::new("mousedown").unwrap());
        })
    };

    {
        let counter = props.counter.clone();
        use_effect_with_deps(
            move |role_delete| {
                if role_delete.data.is_some() {
                    counter.increase();
                }
                || ()
            },
            role_delete,
        );
    };

    html!(
        <form {onsubmit}>
            <div class="modal-body">
                { format!("Do you really want to remove role \"{}\"", role.name) }
            </div>
            <div class="modal-footer">
              <button type="button" onclick={&props.close} class="btn btn-secondary">{"Cancel"}</button>
              <button type="submit" class="btn btn-danger">{"Confirm"}</button>
            </div>
        </form>
    )
}