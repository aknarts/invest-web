use web_sys::{HtmlInputElement, MouseEvent};
use yew::prelude::*;
use yew::{html, Callback, Html};
use yew_hooks::UseCounterHandle;
use time::macros::format_description;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub close: Callback<MouseEvent>,
    pub counter: UseCounterHandle,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InvestmentInfo {
    pub name: String,
    pub maturity: time::Date,
    pub expiration: time::Date,
    pub description: String,
}

#[function_component(ManageInvestment)]
pub fn manage_investment(_props: &Props) -> Html {
    let investment_info = use_state(|| InvestmentInfo {
        name: String::new(),
        maturity: time::Date::MIN,
        expiration: time::Date::MIN,
        description: String::new(),
    });
    let info = (*investment_info).clone();
    let format = format_description!("[year]-[month]-[day]");

    let oninput_name = {
        let investment_info = investment_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*investment_info).clone();
            info.name = input.value();
            investment_info.set(info);
        })
    };

    let oninput_description = {
        let investment_info = investment_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*investment_info).clone();
            info.description = input.value();
            investment_info.set(info);
        })
    };

    let oninput_maturity = {
        let investment_info = investment_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*investment_info).clone();
            if let Ok(d) = time::Date::parse(&input.value(), &format) { info.maturity = d; };
            investment_info.set(info);
        })
    };

    let oninput_expiration = {
        let investment_info = investment_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*investment_info).clone();
            if let Ok(d) = time::Date::parse(&input.value(), &format) { info.maturity = d; };
            investment_info.set(info);
        })
    };

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
                                value={info.name.clone()}
                                oninput={oninput_name}
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
                                oninput={oninput_maturity}
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
                                oninput={oninput_expiration}
                                />
                            <label for="investmentexpirationGroup">{"Investment Expiration"}</label>
                        </div>
                    </div>
                    <div class="input-group mb-2">
                        <div class="form-floating">
                          <textarea class="form-control" placeholder="Leave a comment here" id="floatingTextarea2" style="height: 100px"
                                oninput={oninput_description}>{info.description.clone()}</textarea>
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
