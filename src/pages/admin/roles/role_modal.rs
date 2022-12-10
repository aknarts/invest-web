use std::collections::HashSet;
use tracing::debug;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::{use_async, UseCounterHandle};
use super::permission::PermissionList;
use crate::services::admin::{create_role, edit_role, Role, RoleInfo};

#[derive(Properties, PartialEq)]
pub struct ManageRoleProps {
    pub role: Option<Role>,
    pub close: Callback<MouseEvent>,
    pub counter: UseCounterHandle,
}

#[function_component(ManageRole)]
pub fn manage_role(props: &ManageRoleProps) -> Html {
    let role = props.role.as_ref().map_or(
        Role {
            id: -1,
            name: String::new(),
            description: String::new(),
            permissions: None,
        },
        Clone::clone,
    );
    let r = role.clone();

    let is_edit = props.role.is_some();

    let role_info = use_state(|| RoleInfo {
        id: if r.id == -1 { None } else { Some(r.id) },
        name: r.name,
        description: r.description,
        permissions: r.permissions.map_or_else(HashSet::default, |v| {
            v.iter().map(|x| x.name.clone()).collect()
        }),
    });

    let search_term = use_state(|| None::<String>);

    let role_create = {
        let role_info = role_info.clone();
        use_async(async move {
            let request = (*role_info).clone();
            create_role(request).await
        })
    };

    let role_edit = {
        let role_info = role_info.clone();
        use_async(async move {
            let request = (*role_info).clone();
            edit_role(request).await
        })
    };

    let onsubmit = {
        let role_info = role_info.clone();
        let role = role.clone();
        let close = props.close.clone();
        let role_edit = role_edit.clone();
        let role_create = role_create.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            if is_edit {
                role_edit.run();
            } else {
                role_create.run();
            }
            close.emit(MouseEvent::new("mousedown").unwrap());
            role_info.set(RoleInfo {
                id: if role.id == -1 { None } else { Some(role.id) },
                name: role.name.clone(),
                description: role.description.clone(),
                permissions: role.permissions.clone().map_or_else(HashSet::default, |v| {
                    v.iter().map(|x| x.name.clone()).collect()
                }),
            });
        })
    };

    {
        let counter = props.counter.clone();
        use_effect_with_deps(
            move |role_edit| {
                if role_edit.data.is_some() {
                    counter.increase();
                }
                || ()
            },
            role_edit,
        );
    };

    {
        let counter = props.counter.clone();
        use_effect_with_deps(
            move |role_create| {
                if role_create.data.is_some() {
                    counter.increase();
                }
                || ()
            },
            role_create,
        );
    }

    let fallback = html!(
        <div class="d-flex justify-content-center">
            <span class="spinner-border text-secondary" role="status">
              <span class="sr-only">{"Loading..."}</span>
            </span>
        </div>
    );

    let search = (*search_term).as_ref().cloned();

    let info = (*role_info).clone();

    let oninput_rolename = {
        let role_info = role_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*role_info).clone();
            info.name = input.value();
            role_info.set(info);
        })
    };

    let oninput_description = {
        let role_info = role_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*role_info).clone();
            info.description = input.value();
            role_info.set(info);
        })
    };

    let oninput_permission = {
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*role_info).clone();
            debug!("Input: {}", input.value());

            if info.permissions.contains(&input.value()) {
                info.permissions.remove(&input.value());
            } else {
                info.permissions.insert(input.value());
            }
            debug!("Info: {:?}", info);
            role_info.set(info);
        })
    };

    let oninput_search = {
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if input.value().is_empty() {
                search_term.set(None);
            } else {
                search_term.set(Some(input.value()));
            }
        })
    };

    html!(
        <form {onsubmit}>
            <div class="modal-body">
                <fieldset>
                    <div class="input-group mb-2">
                        <span class="input-group-text">
                          <i class="fas fa-signature"></i>
                        </span>
                        <div class="form-floating">
                            <input
                                class="form-control"
                                type="text"
                                id="rolenameGroup"
                                placeholder="Rolename"
                                value={info.name.clone()}
                                oninput={oninput_rolename}
                                />
                            <label for="rolenameGroup">{"Role Name"}</label>
                        </div>
                    </div>
                    <div class="input-group mb-2">
                        <div class="form-floating">
                            <input
                                class="form-control"
                                type="text"
                                id="rolenameGroup"
                                placeholder="Description"
                                value={info.description.clone()}
                                oninput={oninput_description}
                                />
                            <label for="rolenameGroup">{"Description"}</label>
                        </div>
                    </div>
                    <div class="input-group mb-2">
                        <span class="input-group-text">
                          <i class="fas fa-search"></i>
                        </span>
                        <input
                            class="form-control"
                            type="text"
                            id="search"
                            placeholder="Search"
                            oninput={oninput_search}
                            />
                    </div>
                    <Suspense {fallback}>
                        <PermissionList additional_permissions={role.permissions} selected_callback={oninput_permission} search={search} info={info}/>
                    </Suspense>
                </fieldset>
            </div>
            <div class="modal-footer">
                <button type="submit" class="btn btn-primary">{ if props.role.is_some() {"Edit"} else {"Create"} }</button>
            </div>
        </form>
    )
}