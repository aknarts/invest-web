use crate::app::Route;
use crate::components::list_errors::ListErrors;
use crate::hooks::use_user_context;
use crate::services::auth::*;
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
        use_async(async move { register((*register_info).clone()).await })
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
        Callback::from(move |e: FocusEvent| {
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
        <div class="columns is-mobile is-centered">
            <div class="column is-half">
                <div class="box">
                    <div class="tile is-child hero">
                        <h1 class="title is-1">{ "Sign Up" }</h1>
                        <p class="subtitle">
                            <Link<Route> to={Route::Login}>
                                { "Have an account?" }
                            </Link<Route>>
                        </p>
                    </div>
                    <div class="tile is-child">
                        <ListErrors error={user_register.error.clone()} />
                        <form {onsubmit}>
                            <fieldset>
                                <div class="field">
                                    <div class="control has-icons-left">
                                    <input
                                        class="input"
                                        type="text"
                                        placeholder="Username"
                                        autocomplete="username"
                                        value={register_info.username.clone()}
                                        oninput={oninput_username}
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
                                        type="email"
                                        placeholder="Email"
                                        autocomplete="email"
                                        value={register_info.email.clone()}
                                        oninput={oninput_email}
                                        />
                                        <span class="icon is-small is-left">
                                          <i class="fas fa-envelope"></i>
                                        </span>
                                    </div>
                                </div>
                                <div class="field">
                                    <div class="control has-icons-left">
                                    <input
                                        class="input"
                                        type="password"
                                        placeholder="Password"
                                        autocomplete="new_password"
                                        value={register_info.password.clone()}
                                        oninput={oninput_password}
                                        />
                                        <span class="icon is-small is-left">
                                          <i class="fas fa-lock"></i>
                                        </span>

                                    </div>
                                </div>
                                <div class="field is-grouped is-grouped-right">
                                    <div class="control">
                                        <button
                                            class="button is-success"
                                            type="submit"
                                            disabled=false>
                                            { "Sign up" }
                                        </button>
                                    </div>
                                </div>
                            </fieldset>
                        </form>
                    </div>
                </div>
            </div>
        </div>
    }
}
