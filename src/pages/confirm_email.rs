use crate::error::Error;
use crate::hooks::use_user_context;
use crate::services::auth::confirm_email;
use gloo::history::AnyHistory;
use log::{debug, error};
use serde::Deserialize;
use yew::prelude::*;
use yew::{html, Properties};
use yew_hooks::{use_async, use_async_with_options, use_mount, UseAsyncHandle, UseAsyncOptions};
use yew_router::history::{BrowserHistory, History};

#[derive(Deserialize, Properties, Eq, PartialEq)]
pub struct Props {
    pub code: String,
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Status {
    Unsent,
    Invalid,
    Bad,
    Sent,
    Confirmed,
    Rejected,
}

#[function_component(ConfirmEmail)]
pub fn confirm() -> Html {
    let history = AnyHistory::from(BrowserHistory::new());
    let location = history.location();
    let props = location.query::<Props>();
    let user_ctx = use_user_context();

    let confirmation: UseStateHandle<Status> = use_state(|| Status::Unsent);

    let confirmed = *confirmation;

    let do_confirm = {
        let code = match confirmed {
            Status::Unsent => match &props {
                Ok(d) => Some(d.code.clone()),
                Err(_) => {
                    confirmation.set(Status::Invalid);
                    None
                }
            },
            _ => None,
        };
        error!("Got code {:?}", code);
        use_async(async move {
            if let Some(c) = code {
                debug!("awaiting");
                confirm_email(&c).await
            } else {
                debug!("No code");
                Err(Error::RequestError)
            }
        })
    };
    {
        let do_confirm = do_confirm.clone();
        use_mount(move || {
            do_confirm.run();
        });
    }

    use_effect_with_deps(
        move |do_confirm| {
            if let Some(r) = &do_confirm.data {
                debug!("validation result: {:?}", r);
                if r.result.eq("ok") {
                    user_ctx.validate_email(true);
                    confirmation.set(Status::Confirmed);
                } else {
                    user_ctx.validate_email(false);
                    confirmation.set(Status::Rejected);
                }
            }

            if let Some(error) = &do_confirm.error {
                if let Error::BadRequest = error {
                    confirmation.set(Status::Bad);
                }
            }
            || ()
        },
        do_confirm.clone(),
    );

    html! {
        <div class="tile is-ancestor is-vertical">
            <div class="content has-text-centered">
                {
                    match confirmed {
                        Status::Bad => {{ "Invalid confirmation code. Email already verified?" }}
                        Status::Invalid => {{ "No confirmation code provided" }}
                        Status::Unsent => {{ "Pending confirmation" }}
                        Status::Sent => {{ "Waiting for confirmation" }}
                        Status::Confirmed => {{ "Email confirmed" }}
                        Status::Rejected => {{ "Error confirming the email" }}
                    }
                }
            </div>
        </div>
    }
}
