use super::delete_investment_modal::DeleteInvestment;
use super::modal::ManageInvestment;
use crate::app::Route;
use crate::components::modal::Modal;
use crate::components::table::types::Table;
use crate::components::table::types::{ColumnBuilder, TableData};
use crate::components::table::Options;
use crate::error::Error;
use crate::hooks::use_user_context;
use crate::services::admin::{get_investments_list, Investment};
use crate::services::requests::API_ROOT;
use crate::types::WrapCounter;
use serde::Serialize;
use serde_value::Value;
use tracing::debug;
use web_sys::{HtmlInputElement, InputEvent};
use yew::prelude::*;
use yew::suspense::{use_future, SuspensionResult, UseFutureHandle};
use yew::{html, use_state, Callback, HtmlResult};
use yew_hooks::UseCounterHandle;
use yew_router::prelude::use_navigator;

#[hook]
fn use_investments_list() -> SuspensionResult<UseFutureHandle<Result<Vec<Investment>, Error>>> {
    use_future(|| async move { get_investments_list().await })
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub counter: UseCounterHandle,
}

#[function_component(InvestmentsList)]
pub fn investments_list(props: &Props) -> HtmlResult {
    let res = use_investments_list()?;
    let user_ctx = use_user_context();
    let history = use_navigator().unwrap();
    let active = use_state(|| false);
    let search_term = use_state(|| None::<String>);
    let search = (*search_term).as_ref().cloned();
    let act = active.clone();

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
            let columns = vec![
                ColumnBuilder::new("id")
                    .orderable(true)
                    .short_name("#")
                    .data_property("id")
                    .header_class("user-select-none")
                    .build(),
                ColumnBuilder::new("name")
                    .orderable(true)
                    .short_name("Name")
                    .data_property("name")
                    .header_class("user-select-none")
                    .build(),
                ColumnBuilder::new("location")
                    .orderable(true)
                    .short_name("Location")
                    .data_property("location")
                    .header_class("user-select-none")
                    .build(),
                ColumnBuilder::new("p.a. rate")
                    .orderable(true)
                    .short_name("p.a. rate")
                    .data_property("rate")
                    .header_class("user-select-none")
                    .build(),
                ColumnBuilder::new("Actions")
                    .data_property("actions")
                    .header_class("user-select-none")
                    .build(),
            ];

            let mut data = Vec::new();
            for investment in list {
                data.push(InvestmentLine {
                    id: investment.id,
                    name: investment.name.clone(),
                    location: investment.location.clone(),
                    rate: crate::utils::investments::calculate_rate(
                        investment.value.unwrap_or_default(),
                        investment.earning.unwrap_or_default(),
                    ),
                    investment: investment.clone(),
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
                    if user_ctx.check_permission("create_investment") {
                        <div class="flex-shrink-0 p-2">
                            <button type="button" onclick={&onclick} class="btn btn-success">{ "Add Investment" }</button>
                        </div>
                        if *act {
                            <Modal close={&onclick} active={act} size={crate::components::modal::Size::Large} title="Add new investment" >
                                <ManageInvestment close={&onclick} counter={props.counter.clone()}/>
                            </Modal>
                        }
                    }
                    </div>
                    <Table<InvestmentLine> {options} {search} classes={classes!("table", "table-hover")} columns={columns} data={data} orderable={true}/>
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

#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Serialize)]
struct InvestmentLine {
    pub id: i32,
    pub name: String,
    pub location: Option<String>,
    pub rate: f64,
    #[serde(skip_serializing)]
    pub investment: Investment,
    #[serde(skip_serializing)]
    pub counter: WrapCounter,
}

impl TableData for InvestmentLine {
    fn get_field_as_html(&self, field_name: &str) -> crate::components::table::error::Result<Html> {
        Ok(match field_name {
            "id" => self.investment.pictures.as_ref().map_or_else(|| html!(<span>{ &self.id }</span>), |p| p.first().map_or_else(|| html!(<span>{ &self.id }</span>), |thumb| html!(<span class="d-flex align-items-center">
                     <span style={ format!("width: 90px; \
                              height: 90px; \
                              background-size: cover; \
                              display: inline-block; \
                              background-position: center; \
                              background-repeat: no-repeat; \
                              background-image:url('{API_ROOT}/{}')", thumb.0.replace(".jpg", "_thumb.jpg")) }>
                     </span>
                   </span>
             ))),
            "name" => html!({ &self.name }),
            "rate" => {
                let rate = crate::utils::investments::calculate_pa_earnings(
                    self.investment.value.unwrap_or_default(),
                    self.investment.earning.unwrap_or_default(),
                )
                .unwrap_or_else(|| String::from("0"));
                html!({ format!("{rate} %") })
            }
            "location" => html!({ &self.location.clone().unwrap_or_default() }),
            "actions" => {
                html!(
                    <>
                        <ActionLine investment={self.investment.clone()} counter={self.counter.0.clone().unwrap()}/>
                    </>
                )
            }
            _ => html!(),
        })
    }

    fn get_field_as_value(
        &self,
        field_name: &str,
    ) -> crate::components::table::error::Result<Value> {
        Ok(match field_name {
            "id" => serde_value::to_value(self.id),
            "name" => serde_value::to_value(&self.name),
            "rate" => serde_value::to_value(self.rate),
            "location" => serde_value::to_value(&self.location),
            &_ => serde_value::to_value(""),
        }
        .unwrap())
    }

    fn matches_search(&self, needle: Option<String>) -> bool {
        needle.map_or(true, |search| {
            self.name.to_lowercase().contains(&search.to_lowercase())
        })
    }
}

#[derive(Properties, PartialEq)]
pub struct ActionLineProp {
    pub investment: Investment,
    pub counter: UseCounterHandle,
}

#[function_component(ActionLine)]
fn action_line(props: &ActionLineProp) -> Html {
    let edit = use_state(|| false);
    let ed = edit.clone();
    let delete = use_state(|| false);
    let del = delete.clone();

    let onclick = {
        Callback::from(move |_| {
            edit.set(!*edit);
        })
    };

    let onclick_delete = {
        Callback::from(move |_| {
            delete.set(!*delete);
        })
    };

    html!(
        <>
            <button type="button" onclick={&onclick} class="btn btn-primary mx-1">{ "Edit" }</button>
            <button type="button" onclick={&onclick_delete} class="btn btn-danger mx-1">{"Remove"}</button>
            if *ed {
                <Modal close={&onclick} active={ed} title="Edit Investment" >
                    <ManageInvestment investment={props.investment.clone()} close={&onclick} counter={props.counter.clone()}/>
                </Modal>
            }
            if *del {
                <Modal close={&onclick_delete} active={del} title="Delete investment" >
                    <DeleteInvestment investment={props.investment.clone()} close={&onclick_delete} counter={props.counter.clone()}/>
                </Modal>
            }
        </>
    )
}
