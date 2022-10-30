use std::future::Future;
use log::warn;
use yew::prelude::*;
use yew::suspense::{Suspension, SuspensionResult};
use yew_hooks::{use_async, use_mount};
use crate::error::Error;
use crate::services::admin::{get_user_list, User};
use crate::services::requests::get_token;


#[hook]
fn use_user_list() -> SuspensionResult<Vec<User>> {
    let list_users = use_async(async move { get_user_list().await });
    let data_handle = use_state(
        || {
            let (s, handle) = Suspension::new();
            (Err(s), Some(handle))
        }
    );
    let users = vec!{};
    let users = use_state(|| vec![]);
    {
        let list_users = list_users.clone();
        use_mount(move || {
            if get_token().is_some() {
                list_users.run();
            }
        });
    }

    {
        let user_ctx = user_ctx.clone();
        let users = users.clone();
        let data_handle = data_handle.clone();
        use_effect_with_deps(
            move |list_users| {

                if let Some(users) = &list_users.data {

                }

                if let Some(error) = &list_users.error {
                    if let Error::Unauthorized(s) | Error::Forbidden(s) = error {
                        warn!("Unauthorized {s}");
                    }
                    handle.resume();
                    Err(s);
                }
                || {
                    handle.resume();
                    Err(s);
                }
            },
            list_users,
        );
    };
}


#[function_component(Users)]
pub fn users() -> Html {
    html! {
        <section class="grid flex-fill border-end border-start border-bottom">
            <div>
                <table class="table table-hover">
                  <thead>
                    <tr>
                      <th scope="col">{"#"}</th>
                      <th scope="col">{"Username"}</th>
                      <th scope="col">{"Email"}</th>
                      <th scope="col">{"Actions"}</th>
                    </tr>
                  </thead>
                </table>
            </div>
        </section>
    }
}
