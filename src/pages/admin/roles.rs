use crate::app::Route;
use crate::components::modal::Modal;
use crate::error::Error;
use crate::hooks::use_user_context;
use crate::services::admin::{get_permissions_list, get_role_list, Permission, Role};
use log::debug;
use std::collections::HashSet;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::suspense::{use_future, SuspensionResult, UseFutureHandle};
use yew_router::hooks::use_navigator;

#[hook]
fn use_roles_list() -> SuspensionResult<UseFutureHandle<Result<Vec<Role>, Error>>> {
    use_future(|| async move { get_role_list().await })
}

#[function_component(RoleList)]
pub fn role_list() -> HtmlResult {
    let res = use_roles_list()?;
    let history = use_navigator().unwrap();
    let html_result = match *res {
        Ok(ref list) => {
            html! (
                <div>
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
                                        html!(<RoleLine role={role.clone()} />)
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

#[derive(Properties, PartialEq, Eq)]
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
                <ManageRole role={props.role.clone()} />
            </Modal>
          </td>
        </tr>
    )
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RoleInfo {
    name: String,
    permission: HashSet<String>,
}

#[function_component(Roles)]
pub fn roles() -> Html {
    let active = use_state(|| false);
    let user_ctx = use_user_context();

    let fallback = html! (
        <div class="d-flex justify-content-center">
            <span class="spinner-border text-secondary" role="status">
              <span class="sr-only">{"Loading..."}</span>
            </span>
        </div>
    );

    let act = *active;

    let onclick = {
        Callback::from(move |_| {
            active.set(!*active);
        })
    };

    html! (
        <section class="grid flex-fill border-end border-start border-bottom">
            if user_ctx.check_permission("create_role") {
                <div class="d-flex flex-row-reverse m-1">
                    <button type="button" onclick={&onclick} class="btn btn-success">{ "Add Role" }</button>
                </div>
                <Modal close={&onclick} active={act} title="Create new role" >
                    <ManageRole />
                </Modal>
            }
            <Suspense {fallback}>
                <RoleList />
            </Suspense>
        </section>
    )
}

#[derive(Properties, PartialEq, Eq)]
pub struct ManageRoleProps {
    pub role: Option<Role>,
}

#[function_component(ManageRole)]
pub fn manage_role(props: &ManageRoleProps) -> Html {
    let role = props.role.as_ref().map_or_else(Role::default, Clone::clone);
    let r = role.clone();

    let role_info = use_state(|| RoleInfo {
        name: r.name,
        permission: r.permissions.map_or_else(HashSet::default, |v| {
            v.iter().map(|x| x.name.clone()).collect()
        }),
    });

    let search_term = use_state(|| None::<String>);

    let onsubmit = {
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
        })
    };

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

    let oninput_permission = {
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*role_info).clone();
            debug!("Input: {}", input.value());

            if info.permission.contains(&input.value()) {
                info.permission.remove(&input.value());
            } else {
                info.permission.insert(input.value());
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
                                value={role.name}
                                oninput={oninput_rolename}
                                />
                            <label for="rolenameGroup">{"Role Name"}</label>
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
                <button type="button" class="btn btn-primary">{ if props.role.is_some() {"Edit"} else {"Create"} }</button>
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
                                                        props.info.permission.contains(&permission.name),
                                                        permission)
                                ), |s| if permission.description.contains(s) || permission.name.contains(s) {
                                    permission_checkbox(&props.selected_callback,
                                                        props.info.permission.contains(&permission.name),
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
