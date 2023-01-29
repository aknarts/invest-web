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
pub fn manage_investment(_props: &Props) -> Html {
    html!(
        <>
            <div class="modal-body">
                <span class="h5">
                    {"General"}
                </span>
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
                    <div class="input-group mb-2">
                        <span class="input-group-text">
                          <i class="fas fa-calendar-days"></i>
                        </span>
                        <div class="form-floating">
                            <input
                                class="form-control"
                                type="date"
                                id="investmentmaturityGroup"
                                placeholder="Investment Maturity"
                                />
                            <label for="investmentmaturityGroup">{"Investment Maturity"}</label>
                        </div>
                        <span class="input-group-text">
                          <i class="fas fa-calendar-days"></i>
                        </span>
                        <div class="form-floating">
                            <input
                                class="form-control"
                                type="date"
                                id="investmentexpirationGroup"
                                placeholder="Investment Expiration"
                                />
                            <label for="investmentexpirationGroup">{"Investment Expiration"}</label>
                        </div>
                    </div>
                    <div class="input-group mb-2">
                        <div class="form-floating">
                          <textarea class="form-control" placeholder="Leave a comment here" id="floatingTextarea2" style="height: 100px"></textarea>
                          <label for="floatingTextarea2">{"Description"}</label>
                        </div>
                    </div>
                    <div class="h7">
                        {"Tags"}
                    </div>
                    <div class="container-fluid p-2">
                        <span class="badge rounded-pill bg-secondary">{"rental"}<button class="btn btn-sm m-0 py-0 px-1">{"x"}</button></span>
                    </div>
                    <div class="input-group mb-3 input-group-sm">
                        <span class="input-group-text">{"Add tag"}</span>
                        <input type="text" class="form-control" />
                    </div>
                </fieldset>
                <span class="h5">
                    {"Financials"}
                </span>
                <fieldset>
                    <div class="input-group mb-2">
                        <span class="input-group-text">
                          <i class="fas fa-coins"></i>
                        </span>
                        <div class="form-floating">
                            <input
                                class="form-control"
                                type="number"
                                id="investmentvalueGroup"
                                placeholder="0"
                                />
                            <label for="investmentvalueGroup">{"Investment Value"}</label>
                        </div>
                    </div>
                    <div class="h7">
                        {"Costs"}
                    </div>
                    <div class="input-group mb-2">
                        <div class="input-group-text">
                            <input
                                type="text"
                                placeholder="Cost Name"
                                />
                        </div>
                        <input
                            class="form-control"
                            type="number"
                            placeholder="0"
                            />
                        <button type="button" class="btn btn-success">{"+"}</button>
                    </div>
                </fieldset>
                <div class="h5">
                    {"Pictures"}
                </div>
                <button type="button" class="btn btn-outline-secondary btn-lg px-5 py-5">{"+"}</button>
            </div>
            <div class="modal-footer">
                <button type="submit" class="btn btn-primary">{"Create"}</button>
            </div>
        </>
    )
}
