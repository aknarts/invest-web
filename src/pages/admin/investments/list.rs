use super::modal::ManageInvestment;
use crate::app::Route;
use crate::components::modal::Modal;
use crate::error::Error;
use crate::hooks::use_user_context;
use crate::services::admin::{get_investments_list, Role};
use tracing::debug;
use web_sys::{HtmlInputElement, InputEvent};
use yew::prelude::*;
use yew::suspense::{use_future, SuspensionResult, UseFutureHandle};
use yew::{html, use_state, Callback, HtmlResult};
use yew_hooks::UseCounterHandle;
use yew_router::prelude::use_navigator;

#[hook]
fn use_investments_list() -> SuspensionResult<UseFutureHandle<Result<Vec<Role>, Error>>> {
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
        Ok(ref _list) => {
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
                        <Modal close={&onclick} active={act} size={crate::components::modal::Size::Large} title="Add new investment" >
                            <ManageInvestment close={&onclick} counter={props.counter.clone()}/>
                        </Modal>
                    }
                    </div>

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
