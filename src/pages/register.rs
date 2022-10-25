use crate::app::Route;
use crate::components::list_errors::ListErrors;
use crate::hooks::use_user_context;
use crate::types::auth::RegisterInfo;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::prelude::*;

#[function_component(Register)]
pub fn register() -> Html {
    let user_ctx = use_user_context();
    let register_info = use_state(RegisterInfo::default);
    let user_register = {
        let register_info = register_info.clone();
        use_async(async move { crate::services::auth::register((*register_info).clone()).await })
    };

    {
        use_effect_with_deps(
            move |user_register| {
                if let Some(user_info) = &user_register.data {
                    user_ctx.register(user_info.clone());
                }
                || ()
            },
            user_register.clone(),
        );
    }

    let onsubmit = {
        let user_register = user_register.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default(); /* Prevent event propagation */
            user_register.run();
        })
    };
    let oninput_username = {
        let register_info = register_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*register_info).clone();
            info.username = input.value();
            register_info.set(info);
        })
    };
    let oninput_email = {
        let register_info = register_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*register_info).clone();
            info.email = input.value();
            register_info.set(info);
        })
    };
    let oninput_password = {
        let register_info = register_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*register_info).clone();
            info.password = input.value();
            register_info.set(info);
        })
    };

    html! {
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
                                    disabled=false>
                                    { "Sign up" }
                                </button>
                            </div>
                        </fieldset>
                    </form>
                </div>
            </div>
        </div>
    }
}
