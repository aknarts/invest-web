use crate::hooks::use_user_context_from_ctx;
use crate::services::auth::confirm_email;
use log::error;
use serde::Deserialize;
use yew::prelude::*;
use yew::{html, Component, Properties};
use yew_router::prelude::*;

pub enum Msg {
    Confirmed,
    NotConfirmed,
}

#[derive(Deserialize, Properties, Eq, PartialEq)]
pub struct Props {
    pub code: String,
}

pub struct ConfirmEmail {
    confirmed: Option<bool>,
    code: Option<String>,
}

impl Component for ConfirmEmail {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let history = ctx.link().history().unwrap();
        let location = history.location();
        let props = location.query::<Props>();

        let new = Self {
            confirmed: None,
            code: match props {
                Ok(d) => Some(d.code),
                Err(_) => None,
            },
        };
        match &new.code {
            None => {}
            Some(c) => {
                let code = c.clone();
                ctx.link().send_future(async move {
                    match confirm_email(&code).await {
                        Ok(r) => {
                            if r.result.eq("ok") {
                                Msg::Confirmed
                            } else {
                                Msg::NotConfirmed
                            }
                        }
                        Err(e) => {
                            error!("Failed to confirm email: {}", e);
                            Msg::NotConfirmed
                        }
                    }
                });
            }
        }

        new
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Confirmed => self.confirmed = Some(true),
            Msg::NotConfirmed => self.confirmed = Some(false),
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let user_ctx = use_user_context_from_ctx::<Self>(ctx);
        user_ctx.validate_email(self.confirmed.unwrap_or(false));
        match &self.code {
            None => {
                html! {
                    <div class="tile is-ancestor is-vertical">
                        <div class="content has-text-centered">
                            { " No confirmation code providedd " }
                        </div>
                    </div>
                }
            }
            Some(_code) => {
                html! {
                    <div class="tile is-ancestor is-vertical">
                        <div class="content has-text-centered">
                            {
                                match self.confirmed {
                                    None => {
                                        { "Waiting for confirmation" }
                                    }
                                        Some(v) => {
                                        if v {
                                            { "Email confirmed" }
                                        } else {
                                            { "Error confirming the email" }
                                        }
                                    }
                                }
                            }
                        </div>
                    </div>
                }
            }
        }
    }
}
