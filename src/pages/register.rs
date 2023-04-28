use crate::app::Route;
use crate::components::list_errors::ListErrors;
use crate::hooks::{use_user_context, Routes};
use crate::types::auth::RegisterInfo;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::prelude::*;

#[function_component(Register)]
pub fn register() -> Html {
    let user_ctx = use_user_context();
    let register_info = use_state(RegisterInfo::default);
    let button_state = use_state(|| true);
    let registering_state = use_state(|| false);
    let registering_status = *registering_state;
    let button_status = *button_state;

    if user_ctx.is_authenticated() {
        user_ctx.navigate_to(&Routes::Default(Route::Overview));
    }

    let user_register = {
        let register_info = register_info.clone();
        use_async(async move { crate::services::auth::register((*register_info).clone()).await })
    };

    {
        let bt = button_state.clone();
        let rs = registering_state.clone();
        use_effect_with_deps(
            move |user_register| {
                user_register.data.as_ref().map_or_else(
                    || {
                        if user_register.error.is_some() {
                            bt.set(false);
                            rs.set(false);
                        }
                    },
                    |user_info| {
                        bt.set(false);
                        rs.set(false);
                        user_ctx.register(user_info.clone());
                    },
                );
            },
            user_register.clone(),
        );
    }

    let onsubmit = {
        let user_register = user_register.clone();
        let button_state = button_state.clone();
        Callback::from(move |e: SubmitEvent| {
            button_state.set(true);
            registering_state.set(true);
            e.prevent_default(); /* Prevent event propagation */
            user_register.run();
        })
    };
    let oninput_username = {
        let register_info = register_info.clone();
        let button_state = button_state.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*register_info).clone();
            info.username = input.value();
            button_state
                .set(info.email.is_empty() || info.password.is_empty() || info.username.is_empty());
            register_info.set(info);
        })
    };
    let oninput_email = {
        let register_info = register_info.clone();
        let button_state = button_state.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*register_info).clone();
            info.email = input.value();
            button_state
                .set(info.email.is_empty() || info.password.is_empty() || info.username.is_empty());
            register_info.set(info);
        })
    };
    let oninput_password = {
        let register_info = register_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*register_info).clone();
            info.password = input.value();
            button_state
                .set(info.email.is_empty() || info.password.is_empty() || info.username.is_empty());
            register_info.set(info);
        })
    };

    html!(
        <div class="card shadow align-self-center flex-fill rounded">
            <div class="card-header d-flex">
                <h2 class="card-title p-2 flex-grow-1">{ "Sign Up" }</h2>
                <span class="card-subtitle align-self-center">
                    <Link<Route> to={Route::Login} classes="btn btn-secondary">
                        { "Have an account?" }
                    </Link<Route>>
                </span>
            </div>
            <div class="card-body">
                <div class="card-text">
                    <ListErrors error={user_register.error.clone()} />
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
                                        value={register_info.username.clone()}
                                        oninput={oninput_username}
                                        />
                                    <label for="usernameGroup">{"Username"}</label>
                                </div>
                            </div>
                            <div class="input-group mb-2">
                                <span class="input-group-text">
                                  <i class="fas fa-envelope"></i>
                                </span>
                                <div class="form-floating">
                                    <input
                                        class="form-control"
                                        type="email"
                                        id="emailGroup"
                                        placeholder="Email"
                                        autocomplete="email"
                                        value={register_info.email.clone()}
                                        oninput={oninput_email}
                                        />
                                    <label for="emailGroup">{"Email"}</label>
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
                                        autocomplete="new_password"
                                        value={register_info.password.clone()}
                                        oninput={oninput_password}
                                        />
                                    <label for="passwordGroup">{"Password"}</label>
                                </div>
                            </div>
                            <div class="d-flex justify-content-end">
                                <button
                                    class="btn btn-primary mb-2"
                                    type="submit"
                                    disabled={ button_status }>
                                    {
                                        if registering_status {
                                            html!(
                                              <span class="spinner-border spinner-border-sm" role="status" aria-hidden="true"></span>
                                            )
                                        } else {
                                            html!()
                                        }
                                    }
                                    { "Sign up" }
                                </button>
                            </div>
                        </fieldset>
                    </form>
                </div>
            </div>
        </div>
    )
}
