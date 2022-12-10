use std::cmp::Ordering;
use tracing::debug;
use serde::{Serialize, Serializer};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::suspense::{SuspensionResult, use_future, UseFutureHandle};
use yew_hooks::UseCounterHandle;
use yew_router::prelude::use_navigator;
use crate::app::Route;
use crate::columns;
use super::role_modal::ManageRole;
use serde_value::Value;
use super::delete_role_modal::DeleteRole;
use crate::components::modal::Modal;
use crate::components::table::types::{Table, TableData};
use crate::services::admin::{get_role_list, Role};
use crate::error::Error;
use crate::hooks::use_user_context;

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
                });
            }

            html!(
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
                        <ActionLine role={self.role.clone()} counter={self.counter.0.clone().unwrap()}/>
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
            "id" => serde_value::to_value(self.role.id),
            "name" => serde_value::to_value(&self.role.name),
            "description" => serde_value::to_value(&self.role.description),
            &_ => serde_value::to_value(""),
        };
        Ok(value.unwrap())
    }

    fn matches_search(&self, needle: Option<String>) -> bool {
        debug!("Searching: {:?}", needle);
        needle.map_or(true, |search| {
            self.name.to_lowercase().contains(&search.to_lowercase())
        })
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