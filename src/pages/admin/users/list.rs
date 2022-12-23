use crate::app::Route;
use crate::components::modal::Modal;
use crate::components::table::types::Table;
use crate::components::table::types::{ColumnBuilder, TableData};
use crate::components::table::Options;
use crate::error::Error;
use crate::pages::admin::users::modal::UserDetails;
use crate::services::admin::{get_user_list, User};
use crate::types::WrapCounter;
use serde::Serialize;
use serde_value::Value;
use tracing::debug;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::suspense::{use_future, SuspensionResult, UseFutureHandle};
use yew_hooks::UseCounterHandle;
use yew_router::hooks::use_navigator;

#[hook]
fn use_user_list() -> SuspensionResult<UseFutureHandle<Result<Vec<User>, Error>>> {
    use_future(|| async move { get_user_list().await })
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub counter: UseCounterHandle,
}

#[function_component(UserList)]
pub fn user_list(props: &Props) -> HtmlResult {
    let res = use_user_list()?;
    let history = use_navigator().unwrap();
    let search_term = use_state(|| None::<String>);
    let search = (*search_term).as_ref().cloned();

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

    let html_result = match *res {
        Ok(ref list) => {
            let columns = vec![
                ColumnBuilder::new("id")
                    .orderable(true)
                    .short_name("#")
                    .data_property("id")
                    .header_class("user-select-none")
                    .build(),
                ColumnBuilder::new("username")
                    .orderable(true)
                    .short_name("Username")
                    .data_property("username")
                    .header_class("user-select-none")
                    .build(),
                ColumnBuilder::new("email")
                    .orderable(true)
                    .short_name("Email")
                    .data_property("email")
                    .header_class("user-select-none")
                    .build(),
                ColumnBuilder::new("Actions")
                    .data_property("actions")
                    .header_class("user-select-none")
                    .build(),
            ];

            let mut data = Vec::new();
            for user in list.iter() {
                data.push(UserLine {
                    id: user.id,
                    username: user.username.clone(),
                    email: user.email.clone(),
                    user: user.clone(),
                    counter: WrapCounter(Some(props.counter.clone())),
                });
            }

            let options = Options {
                unordered_class: Some("fa-sort".to_string()),
                ascending_class: Some("fa-sort-up".to_string()),
                descending_class: Some("fa-sort-down".to_string()),
                orderable_classes: vec!["mx-1".to_string(), "fa-solid".to_string()],
            };

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
                    </div>
                    <Table<UserLine> {options} {search} classes={classes!("table", "table-hover")} columns={columns} data={data} orderable={true}/>
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

#[derive(Default, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
struct UserLine {
    pub id: i64,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub user: User,
    #[serde(skip_serializing)]
    pub counter: WrapCounter,
}

impl TableData for UserLine {
    fn get_field_as_html(&self, field_name: &str) -> crate::components::table::error::Result<Html> {
        let html = match field_name {
            "id" => html!({ &self.id }),
            "username" => html!({ &self.username }),
            "email" => html!({ &self.email }),
            "actions" => {
                html!(
                    <>
                        <ActionLine user={self.user.clone()} counter={self.counter.0.clone().unwrap()}/>
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
            "id" => serde_value::to_value(self.id),
            "username" => serde_value::to_value(&self.username),
            "email" => serde_value::to_value(&self.email),
            &_ => serde_value::to_value(""),
        };
        Ok(value.unwrap())
    }

    fn matches_search(&self, needle: Option<String>) -> bool {
        needle.map_or(true, |search| {
            self.username
                .to_lowercase()
                .contains(&search.to_lowercase())
                || self.email.to_lowercase().contains(&search.to_lowercase())
        })
    }
}

#[derive(Properties, PartialEq)]
pub struct ActionLineProp {
    pub user: User,
    pub counter: UseCounterHandle,
}

#[function_component(ActionLine)]
fn action_line(props: &ActionLineProp) -> Html {
    let details = use_state(|| false);
    let det = details.clone();

    let user = props.user.clone();

    let onclick = {
        Callback::from(move |_| {
            details.set(!*details);
        })
    };

    html!(
        <>
            <Modal close={&onclick} active={det} title={format!("User <mark>{}</mark> details", user.username)} >
                <UserDetails user_id={user.id} close={&onclick} counter={props.counter.clone()}/>
            </Modal>
            <button onclick={&onclick} type="button" class="btn btn-primary mx-1">{ "Details" }</button>
        </>
    )
}
