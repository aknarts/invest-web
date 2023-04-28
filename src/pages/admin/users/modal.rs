use crate::app::Route;
use crate::error::Error;
use crate::services::admin::{get_user_detail, UserDetail};
use crate::types::auth::EmailDetail;
use tracing::debug;
use yew::prelude::*;
use yew::suspense::{use_future, SuspensionResult, UseFutureHandle};
use yew_hooks::{use_async, UseCounterHandle};
use yew_router::prelude::use_navigator;

#[hook]
fn use_user_detail(id: i64) -> SuspensionResult<UseFutureHandle<Result<UserDetail, Error>>> {
    use_future(|| async move { get_user_detail(id).await })
}

#[derive(Properties, PartialEq)]
pub struct UserDetailsProps {
    pub user_id: i64,
    pub close: Callback<MouseEvent>,
    pub counter: UseCounterHandle,
}

#[function_component(UserDetails)]
pub fn user_details(props: &UserDetailsProps) -> Html {
    let fallback = html!(
        <div class="d-flex justify-content-center">
            <span class="spinner-border text-secondary" role="status">
              <span class="sr-only">{"Loading..."}</span>
            </span>
        </div>
    );

    html!(
        <section class="grid flex-fill border-end border-start border-bottom">
            <Suspense {fallback}>
                <Detail user_id={props.user_id} close={props.close.clone()} counter={props.counter.clone()}/>
            </Suspense>
        </section>
    )
}

#[function_component(Detail)]
pub fn detail(props: &UserDetailsProps) -> HtmlResult {
    let res = use_user_detail(props.user_id)?;
    let history = use_navigator().unwrap();

    let html_result = match *res {
        Ok(ref user) => {
            html!(
                <form>
                    <table class="table table-striped-columns">
                        <tbody>
                            <tr>
                                <td>
                                    <label class="col-form-label">{ "Username" }</label>
                                </td>
                                <td>
                                    { user.username.clone() }
                                </td>
                            </tr>
                            <tr>
                                <td class="align-top">
                                    <label class="col-form-label">{ "Emails" }</label>
                                </td>
                                <td>
                                    <table class="table">
                                        { for user.emails.iter().map(|e|
                                            html!(
                                                <EmailRow email={e.clone()} user_id={user.id}/>
                                            )
                                        ) }
                                    </table>
                                </td>
                            </tr>
                            <tr>
                                <td class="align-top">
                                    <label class="col-form-label">{ "Roles" }</label>
                                </td>
                                <td>
                                    <table class="table">
                                        { for user.roles.iter().map(|r|
                                            html!(
                                                <tr>
                                                    <td>
                                                        { r }
                                                    </td>
                                                </tr>
                                            )
                                        ) }
                                    </table>
                                </td>
                            </tr>
                        </tbody>
                    </table>
                </form>
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

#[derive(Properties, Eq, PartialEq)]
pub struct EmailRowProps {
    pub user_id: i64,
    pub email: EmailDetail,
}

#[function_component(EmailRow)]
pub fn email_row(props: &EmailRowProps) -> Html {
    let resent = use_state(|| true);
    let e = props.email.clone();
    let email = e.email.clone();
    let show_resend_notification = *resent;
    let user_id = props.user_id;

    let resend_email = use_async(crate::services::auth::resend(user_id, email));

    let resend = {
        Callback::from(move |_| {
            resent.set(!*resent);
            resend_email.run();
        })
    };

    html!(
        <tr>
            <td class={if !e.verified && show_resend_notification {"text-bg-warning"} else {""}}>
                {
                    if e.verified {
                        if e.primary {
                            html!( <i class="fa-solid fa-circle-check" /> )
                        } else {
                            html!( <i class="fa-solid fa-check" /> )
                        }
                    } else if show_resend_notification {
                            html!( <a onclick={resend}><i class="fa-solid fa-circle-exclamation" /></a> )
                        } else {
                            html!( <i class="fa-solid fa-circle-exclamation" />
                        )
                    }
                }
                { e.email.clone() }
            </td>
        </tr>
    )
}
