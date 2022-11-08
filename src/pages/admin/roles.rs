use crate::app::Route;
use crate::error::Error;
use crate::services::admin::{get_role_list, Role};
use log::debug;
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
            html! {
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
                            for list.iter().map(|role| {role_line(role)})
                        }
                      </tbody>
                    </table>
                    <div class="d-flex flex-row-reverse">
                        <button type="button" class="btn btn-success">{ "Add Role" }</button>
                    </div>
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

fn role_line(role: &Role) -> Html {
    html!(
        <tr>
          <th scope="row">{&role.id}</th>
          <td>{&role.name}</td>
          <td>{&role.description}</td>
          <td><button type="button" class="btn btn-primary mx-1">{ "Edit" }</button><button type="button" class="btn btn-danger mx-1">{"Remove"}</button></td>
        </tr>
    )
}

#[function_component(Roles)]
pub fn roles() -> Html {
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
                <RoleList />
            </Suspense>
        </section>
    }
}
