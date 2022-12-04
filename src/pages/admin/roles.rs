use crate::app::Route;
use crate::components::modal::Modal;
use crate::error::Error;
use crate::hooks::use_user_context;
use crate::services::admin::{
    create_role, edit_role, get_permissions_list, get_role_list, Permission, Role, RoleInfo,
};
use std::collections::HashSet;
use tracing::debug;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::suspense::{use_future, SuspensionResult, UseFutureHandle};
use yew_hooks::use_async;
use yew_router::hooks::use_navigator;

#[hook]
fn use_roles_list() -> SuspensionResult<UseFutureHandle<Result<Vec<Role>, Error>>> {
    use_future(|| async move { get_role_list().await })
}

#[function_component(RoleList)]
pub fn role_list() -> HtmlResult {
    let res = use_roles_list()?;
    let user_ctx = use_user_context();
    let history = use_navigator().unwrap();
    let active = use_state(|| false);

    let act = *active;

    let onclick = {
        Callback::from(move |_| {
            active.set(!*active);
        })
    };

    let html_result = match *res {
        Ok(ref list) => {
            html! (
                <div>
                    if user_ctx.check_permission("create_role") {
                        <div class="d-flex flex-row-reverse m-1">
                            <button type="button" onclick={&onclick} class="btn btn-success">{ "Add Role" }</button>
                        </div>
                        <Modal close={&onclick} active={act} title="Create new role" >
                            <ManageRole close={&onclick} />
                        </Modal>
                    }
                    <table class="table table-hover">
                      <thead>
                        <tr>
                          <th scope="col">{"#"}</th>
                          <th scope="col">{"Name"}</th>
                          <th scope="col">{"Description"}</th>
                          <th scope="col">{"Actions"}</th>
                        </tr>
                      </thead>
                      <tbody>
                        {
                            for list.iter().map(|role| {
                                    if role.name.eq("admin"){
                                        html!()
                                    } else {
                                        html!(<RoleLine role={role.clone()}/>)
                                    }
                                }
                            )
                        }
                      </tbody>
                    </table>
                </div>
            )
        }
        Err(ref e) => {
            match e {
                Error::Unauthorized(s) | Error::Forbidden(s) => {
                    debug!("Authorization issue: {}", s);
                }
                _ => {
                    debug!("Failed to complete request: {e}");
                }
            };
            history.push(&Route::Home);
            html!(500)
        }
    };
    Ok(html_result)
}

#[derive(Properties, PartialEq)]
pub struct RoleLineProp {
    pub role: Role,
}

#[function_component(RoleLine)]
fn role_line(props: &RoleLineProp) -> Html {
    let active = use_state(|| false);
    let act = *active;

    let onclick = {
        Callback::from(move |_| {
            debug!("Clicked");
            active.set(!*active);
        })
    };

    html!(
        <tr>
          <th scope="row">{&props.role.id}</th>
          <td>{&props.role.name}</td>
          <td>{&props.role.description}</td>
          <td>
            <button type="button" onclick={&onclick} class="btn btn-primary mx-1">{ "Edit" }</button><button type="button" class="btn btn-danger mx-1">{"Remove"}</button>
            <Modal close={&onclick} active={act} title="Edit role" >
                <ManageRole role={props.role.clone()} close={&onclick}/>
            </Modal>
          </td>
        </tr>
    )
}

#[function_component(Roles)]
pub fn roles() -> Html {
    let fallback = html! (
        <div class="d-flex justify-content-center">
            <span class="spinner-border text-secondary" role="status">
              <span class="sr-only">{"Loading..."}</span>
            </span>
        </div>
    );

    html! (
        <section class="grid flex-fill border-end border-start border-bottom">
            <Suspense {fallback}>
                <RoleList/>
            </Suspense>
        </section>
    )
}

#[derive(Properties, PartialEq)]
pub struct ManageRoleProps {
    pub role: Option<Role>,
    pub close: Callback<MouseEvent>,
}

#[function_component(ManageRole)]
pub fn manage_role(props: &ManageRoleProps) -> Html {
    let role = props.role.as_ref().map_or(
        Role {
            id: -1,
            name: "".to_string(),
            description: "".to_string(),
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
        use_effect_with_deps(
            move |role_edit| {
                if let Some(_) = &role_edit.data {
                    // force refresh
                }
                || ()
            },
            role_edit,
        )
    };

    {
        use_effect_with_deps(
            move |role_create| {
                debug!("run2");
                if let Some(_) = &role_create.data {
                    // force refresh
                }
                || ()
            },
            role_create,
        );
    }

    let fallback = html! (
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

#[hook]
fn use_permissions_list() -> SuspensionResult<UseFutureHandle<Result<Vec<Permission>, Error>>> {
    use_future(|| async move { get_permissions_list().await })
}

#[derive(Properties, PartialEq)]
pub struct PermissionsProp {
    pub selected_callback: Callback<InputEvent>,
    pub search: Option<String>,
    pub info: RoleInfo,
    pub additional_permissions: Option<Vec<Permission>>,
}

#[function_component(PermissionList)]
pub fn permission_list(props: &PermissionsProp) -> HtmlResult {
    let res = use_permissions_list()?;
    let history = use_navigator().unwrap();
    let html_result = match *res {
        Ok(ref list) => {
            let mut full_list = list.clone();
            if let Some(additional) = &props.additional_permissions {
                for add in additional {
                    if !list.contains(add) {
                        full_list.push(add.clone());
                    }
                }
            }
            full_list.sort_by(|a, b| a.description.cmp(&b.description));

            html!(
                <>
                    {
                        for full_list.iter().map(|permission| {
                            props.search.as_ref().map_or_else(|| html!(
                                    permission_checkbox(&props.selected_callback,
                                                        props.info.permissions.contains(&permission.name),
                                                        permission)
                                ), |s| if permission.description.contains(s) || permission.name.contains(s) {
                                    permission_checkbox(&props.selected_callback,
                                                        props.info.permissions.contains(&permission.name),
                                                        permission)
                                } else {
                                   html!()
                                }
                            )
                        })
                    }
                </>
            )
        }
        Err(ref e) => {
            match e {
                Error::Unauthorized(s) | Error::Forbidden(s) => {
                    debug!("Authorization issue: {}", s);
                }
                _ => {
                    debug!("Failed to complete request: {e}");
                }
            };
            history.push(&Route::Home);
            html!(500)
        }
    };
    Ok(html_result)
}

fn permission_checkbox(
    callback: &Callback<InputEvent>,
    checked: bool,
    permission: &Permission,
) -> Html {
    html!(
        <PermissionLine
           selected_callback={callback}
           checked={ checked }
           name={permission.name.clone()}
           description={permission.description.clone()} />
    )
}

#[derive(Properties, PartialEq)]
pub struct PermissionProp {
    pub selected_callback: Callback<InputEvent>,
    pub checked: bool,
    pub name: String,
    pub description: String,
}

#[function_component(PermissionLine)]
pub fn permission_line(props: &PermissionProp) -> Html {
    html!(
        <div class="form-check form-switch">
            <input
                class="form-check-input"
                type="checkbox"
                id={ props.name.clone() }
                checked={ props.checked }
                value={ props.name.clone() }
                oninput={&props.selected_callback}
                />
            <label class="form-check-label" for={ props.name.clone() }>{ props.description.clone() }</label>
        </div>
    )
}
