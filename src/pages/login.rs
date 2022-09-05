use web_sys::HtmlInputElement;

use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::prelude::*;

use crate::app::Route;
use crate::components::list_errors::ListErrors;
use crate::hooks::use_user_context;
use crate::services::auth::*;
use crate::types::auth::{LoginInfo, LoginInfoWrapper};

/// Login page
#[function_component(Login)]
pub fn login() -> Html {
    let user_ctx = use_user_context();
    let login_info = use_state(LoginInfo::default);
    let user_login = {
        let login_info = login_info.clone();
        use_async(async move {
            let request = LoginInfoWrapper {
                user: (*login_info).clone(),
            };
            login(request).await
        })
    };

    use_effect_with_deps(
        move |user_login| {
            if let Some(user_info) = &user_login.data {
                user_ctx.login(user_info.user.clone());
            }
            || ()
        },
        user_login.clone(),
    );

    let onsubmit = {
        let user_login = user_login.clone();
        Callback::from(move |e: FocusEvent| {
            e.prevent_default(); /* Prevent event propagation */
            user_login.run();
        })
    };
    let oninput_email = {
        let login_info = login_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*login_info).clone();
            info.email = input.value();
            login_info.set(info);
        })
    };
    let oninput_password = {
        let login_info = login_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*login_info).clone();
            info.password = input.value();
            login_info.set(info);
        })
    };

    html! {
        <div class="section">
            <div class="tile is-child hero">
                <h1 class="title is-1">{ "Sign In" }</h1>
                <p class="subtitle">
                    <Link<Route> to={Route::Register}>
                        { "Need an account?" }
                    </Link<Route>>
                </p>
            </div>
            <div class="tile is-child">
                <ListErrors error={user_login.error.clone()} />
                <form {onsubmit}>
                    <fieldset>
                        <div class="field">
                            <div class="control has-icons-left">
                                <input
                                    class="input"
                                    type="email"
                                    placeholder="Email"
                                    value={login_info.email.clone()}
                                    oninput={oninput_email}
                                    />
                                <span class="icon is-small is-left">
                                  <i class="fas fa-user"></i>
                                </span>
                            </div>
                        </div>
                        <div class="field">
                            <div class="control has-icons-left">
                                <input
                                    class="input"
                                    type="password"
                                    placeholder="Password"
                                    value={login_info.password.clone()}
                                    oninput={oninput_password}
                                    />
                                <span class="icon is-small is-left">
                                  <i class="fas fa-lock"></i>
                                </span>
                            </div>
                        </div>
                        <div class="field">
                            <div class="control">
                                <button
                                    class="button is-success"
                                    type="submit"
                                    disabled=false>
                                    { "Sign in" }
                                </button>
                            </div>
                        </div>
                    </fieldset>
                </form>
            </div>
        </div>
    }
}
