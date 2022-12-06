use crate::app::Route;
use crate::columns;
use crate::components::modal::Modal;
use crate::components::table::table::{Table, TableData};
use crate::error::Error;
use crate::hooks::use_user_context;
use crate::services::admin::{
    create_role, edit_role, get_permissions_list, get_role_list, Permission, Role, RoleId, RoleInfo,
};
use serde::{Serialize, Serializer};
use serde_value::Value;
use std::cmp::Ordering;
use std::collections::HashSet;
use tracing::debug;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::suspense::{use_future, SuspensionResult, UseFutureHandle};
use yew_hooks::{use_async, use_counter, UseCounterHandle};
use yew_router::hooks::use_navigator;

#[hook]
fn use_roles_list() -> SuspensionResult<UseFutureHandle<Result<Vec<Role>, Error>>> {
    use_future(|| async move { get_role_list().await })
}

#[derive(Properties, PartialEq)]
pub struct RoleListProp {
    pub counter: UseCounterHandle,
}

#[function_component(RoleList)]
pub fn role_list(props: &RoleListProp) -> HtmlResult {
    let res = use_roles_list()?;
    let user_ctx = use_user_context();
    let history = use_navigator().unwrap();
    let active = use_state(|| false);
    let search_term = use_state(|| None::<String>);
    let search = (*search_term).as_ref().cloned();
    let act = *active;
    debug!("search: {:?}", search);

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

    let onclick = {
        Callback::from(move |_| {
            active.set(!*active);
        })
    };

    let html_result = match *res {
        Ok(ref list) => {
            let columns = columns![("id", "id", "#", true)("name", "Name", "Name", true)(
                "description",
                "Description",
                "Description",
                true
            )("actions", "Actions")];

            let mut data = Vec::new();
            for role in list.iter() {
                if role.name.eq("admin") {
                    continue;
                }
                data.push(RoleLine {
                    id: role.id,
                    name: role.name.clone(),
                    description: role.description.clone(),
                    role: role.clone(),
                    counter: WrapCounter(Some(props.counter.clone())),
                })
            }

            html! (
                <div>
                    <div class="d-flex">
                        <div class="flex-grow-1 p-2 input-group mb-2">
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
                    if user_ctx.check_permission("create_role") {
                        <div class="flex-shrink-0 p-2">
                            <button type="button" onclick={&onclick} class="btn btn-success">{ "Add Role" }</button>
                        </div>
                        <Modal close={&onclick} active={act} title="Create new role" >
                            <ManageRole close={&onclick} counter={props.counter.clone()}/>
                        </Modal>
                    }
                    </div>
                    <Table<RoleLine> {search} classes={classes!("table", "table-hover")} columns={columns} data={data} orderable={true}/>
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

#[derive(Default, Clone)]
struct WrapCounter(Option<UseCounterHandle>);

impl PartialEq<Self> for WrapCounter {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl Eq for WrapCounter {}

impl Serialize for WrapCounter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i8(0)
    }
}

#[derive(Default, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
struct RoleLine {
    pub id: i32,
    pub name: String,
    pub description: String,
    #[serde(skip_serializing)]
    pub role: Role,
    #[serde(skip_serializing)]
    pub counter: WrapCounter,
}

impl Ord for WrapCounter {
    fn cmp(&self, _: &Self) -> Ordering {
        Ordering::Equal
    }
}

impl PartialOrd for WrapCounter {
    fn partial_cmp(&self, _: &Self) -> Option<Ordering> {
        Some(Ordering::Equal)
    }
}

impl TableData for RoleLine {
    fn get_field_as_html(&self, field_name: &str) -> crate::components::table::error::Result<Html> {
        let html = match field_name {
            "id" => html!({ &self.id }),
            "name" => html!({ &self.name }),
            "description" => html!({ &self.description }),
            "actions" => {
                html!(
                    <>
                        <ActionLine role={self.role.clone()} counter={self.counter.0.clone().unwrap().clone()}/>
                    </>
                )
            }
            &_ => {
                html!()
            }
        };
        Ok(html)
    }

    fn get_field_as_value(
        &self,
        field_name: &str,
    ) -> crate::components::table::error::Result<Value> {
        let value = match field_name {
            "id" => serde_value::to_value(&self.role.id),
            "name" => serde_value::to_value(&self.role.name),
            "description" => serde_value::to_value(&self.role.description),
            &_ => serde_value::to_value(""),
        };
        Ok(value.unwrap())
    }

    fn matches_search(&self, needle: Option<String>) -> bool {
        debug!("Searching: {:?}", needle);
        match needle {
            None => {
                return true;
            }
            Some(search) => self.name.to_lowercase().contains(&search.to_lowercase()),
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct ActionLineProp {
    pub role: Role,
    pub counter: UseCounterHandle,
}

#[function_component(ActionLine)]
fn role_line(props: &ActionLineProp) -> Html {
    let edit = use_state(|| false);
    let ed = *edit;
    let delete = use_state(|| false);
    let del = *delete;

    let onclick = {
        Callback::from(move |_| {
            debug!("Clicked");
            edit.set(!*edit);
        })
    };

    let onclick_delete = {
        Callback::from(move |_| {
            debug!("Clicked");
            delete.set(!*delete);
        })
    };

    html!(
        <>
            <button type="button" onclick={&onclick} class="btn btn-primary mx-1">{ "Edit" }</button>
            <button type="button" onclick={&onclick_delete} class="btn btn-danger mx-1">{"Remove"}</button>
            <Modal close={&onclick} active={ed} title="Edit role" >
                <ManageRole role={props.role.clone()} close={&onclick} counter={props.counter.clone()}/>
            </Modal>
            <Modal close={&onclick_delete} active={del} title="Delete role" >
                <DeleteRole role={props.role.clone()} close={&onclick_delete} counter={props.counter.clone()}/>
            </Modal>
        </>
    )
}

#[function_component(Roles)]
pub fn roles() -> Html {
    let counter = use_counter(0);

    let count = *counter;
    debug!("Current count: {}", count);

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
                <RoleList counter={counter} key={count}/>
            </Suspense>
        </section>
    )
}

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
            name: "".to_string(),
            description: "".to_string(),
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
                if let Some(_) = &role_delete.data {
                    counter.increase();
                }
                || ()
            },
            role_delete,
        )
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
        let counter = props.counter.clone();
        use_effect_with_deps(
            move |role_edit| {
                if let Some(_) = &role_edit.data {
                    counter.increase();
                }
                || ()
            },
            role_edit,
        )
    };

    {
        let counter = props.counter.clone();
        use_effect_with_deps(
            move |role_create| {
                if let Some(_) = &role_create.data {
                    counter.increase();
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
