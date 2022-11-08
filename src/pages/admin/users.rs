use crate::app::Route;
use crate::error::Error;
use crate::services::admin::{get_user_list, User};
use log::debug;
use yew::prelude::*;
use yew::suspense::{use_future, SuspensionResult, UseFutureHandle};
use yew_router::hooks::use_navigator;

#[hook]
fn use_user_list() -> SuspensionResult<UseFutureHandle<Result<Vec<User>, Error>>> {
    use_future(|| async move { get_user_list().await })
}

#[function_component(UserList)]
pub fn user_list() -> HtmlResult {
    let res = use_user_list()?;
    let history = use_navigator().unwrap();
    let html_result = match *res {
        Ok(ref list) => {
            html! {
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
                      <tbody>
                        {
                            for list.iter().map(|user| {user_line(user)})
                        }
                      </tbody>
                    </table>
                </div>
            }
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

fn user_line(user: &User) -> Html {
    html!(
        <tr>
          <th scope="row">{&user.id}</th>
          <td>{&user.username}</td>
          <td>{&user.email}</td>
          <td><button type="button" class="btn btn-primary mx-1">{ "Details" }</button></td>
        </tr>
    )
}

#[function_component(Users)]
pub fn users() -> Html {
    let fallback = html! {
        <div class="d-flex justify-content-center">
            <span class="spinner-border text-secondary" role="status">
              <span class="sr-only">{"Loading..."}</span>
            </span>
        </div>
    };
    html! {
        <section class="grid flex-fill border-end border-start border-bottom">
            <Suspense {fallback}>
                <UserList />
            </Suspense>
        </section>
    }
}
