use crate::app::Route;
use crate::components::list_errors::ListErrors;
use crate::hooks::use_user_context;
use crate::types::auth::LoginInfo;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::prelude::*;

/// Login page
#[function_component(Login)]
pub fn login() -> Html {
    let user_ctx = use_user_context();
    let login_info = use_state(LoginInfo::default);
    let user_login = {
        let login_info = login_info.clone();
        use_async(async move {
            let request = (*login_info).clone();
            crate::services::auth::login(request).await
        })
    };

    use_effect_with_deps(
        move |user_login| {
            if let Some(user_info) = &user_login.data {
                user_ctx.login(user_info.clone());
            }
            || ()
        },
        user_login.clone(),
    );

    let onsubmit = {
        let user_login = user_login.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default(); /* Prevent event propagation */
            user_login.run();
        })
    };
    let oninput_username = {
        let login_info = login_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*login_info).clone();
            info.username = input.value();
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
        <div class="card align-self-center flex-fill shadow rounded">
            <div class="card-header d-flex">
                <h2 class="card-title p-2 flex-grow-1">{ "Sign In" }</h2>
                <span class="card-subtitle align-self-center">
                    <Link<Route> to={Route::Register} classes="btn btn-secondary">
                        { "Need an account?" }
                    </Link<Route>>
                </span>
            </div>
            <div class="card-body">
                <div class="card-text">
                    <ListErrors error={user_login.error.clone()} />
                    <form {onsubmit}>
                        <fieldset>
                            <div class="input-group mb-2">
                                <span class="input-group-text">
                                  <i class="fas fa-user"></i>
                                </span>
                                <div class="form-floating">
                                    <input
                                        class="form-control"
                                        type="text"
                                        id="usernameGroup"
                                        placeholder="Username"
                                        autocomplete="username"
                                        value={login_info.username.clone()}
                                        oninput={oninput_username}
                                        />
                                    <label for="usernameGroup">{"Username"}</label>
                                </div>
                            </div>
                            <div class="input-group mb-2">
                                <span class="input-group-text">
                                  <i class="fas fa-lock"></i>
                                </span>
                                <div class="form-floating">
                                    <input
                                        class="form-control"
                                        type="password"
                                        placeholder="Password"
                                        id="passwordGroup"
                                        autocomplete="current_password"
                                        value={login_info.password.clone()}
                                        oninput={oninput_password}
                                        />
                                    <label for="passwordGroup">{"Password"}</label>
                                </div>
                            </div>
                            <div class="d-flex justify-content-end">
                                <button
                                    class="btn btn-primary mb-2"
                                    type="submit"
                                    disabled=false>
                                    { "Sign in" }
                                </button>
                            </div>
                        </fieldset>
                    </form>
                </div>
            </div>
        </div>
    }
}
