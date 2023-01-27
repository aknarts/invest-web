use web_sys::MouseEvent;
use yew::prelude::*;
use yew::{html, Callback, Html};
use yew_hooks::UseCounterHandle;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub close: Callback<MouseEvent>,
    pub counter: UseCounterHandle,
}

#[function_component(ManageInvestment)]
pub fn manage_investment(props: &Props) -> Html {
    html!(
        <form>
            <div class="modal-body">
                <fieldset>
                    <div class="input-group mb-2">
                        <span class="input-group-text">
                          <i class="fas fa-signature"></i>
                        </span>
                        <div class="form-floating">
                            <input
                                class="form-control"
                                type="text"
                                id="investmentnameGroup"
                                placeholder="Investment Name"
                                />
                            <label for="investmentnameGroup">{"Investment Name"}</label>
                        </div>
                    </div>
                </fieldset>
            </div>
            <div class="modal-footer">
                <button type="submit" class="btn btn-primary">{"Create"}</button>
            </div>
        </form>
    )
}
