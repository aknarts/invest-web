use super::costs::{Costs, InvestmentCost};
use super::pictures::Pictures;
use super::tag::Tag;
use std::collections::HashSet;
use time::macros::format_description;
use tracing::debug;
use web_sys::{HtmlInputElement, MouseEvent};
use yew::prelude::*;
use yew::{html, Callback, Html};
use yew_hooks::UseCounterHandle;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub close: Callback<MouseEvent>,
    pub counter: UseCounterHandle,
}

#[derive(Clone, Debug, PartialEq)]
pub struct InvestmentInfo {
    pub name: String,
    pub maturity: time::Date,
    pub expiration: time::Date,
    pub description: String,
    pub tags: HashSet<String>,
    pub value: f64,
    pub costs: Vec<InvestmentCost>,
    pub photos: Vec<String>,
}

#[function_component(ManageInvestment)]
pub fn manage_investment(_props: &Props) -> Html {
    let investment_info = use_state(|| InvestmentInfo {
        name: String::new(),
        maturity: time::Date::MIN,
        expiration: time::Date::MIN,
        description: String::new(),
        tags: HashSet::new(),
        value: 0.0,
        costs: vec![],
        photos: vec![],
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

    let oninput_value = {
        let investment_info = investment_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*investment_info).clone();
            if let Ok(v) = input.value().parse::<f64>() {
                info.value = v;
            }
            investment_info.set(info);
        })
    };

    let oninput_maturity = {
        let investment_info = investment_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*investment_info).clone();
            if let Ok(d) = time::Date::parse(&input.value(), &format) {
                info.maturity = d;
            };
            investment_info.set(info);
        })
    };

    let oninput_expiration = {
        let investment_info = investment_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*investment_info).clone();
            if let Ok(d) = time::Date::parse(&input.value(), &format) {
                info.maturity = d;
            };
            investment_info.set(info);
        })
    };

    let oninput_tags = {
        let investment_info = investment_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*investment_info).clone();
            let current: String = input.value();
            let mut tags = current.split(',').map(str::trim).peekable();

            while let Some(tag) = tags.next() {
                if tags.peek().is_some() {
                    info.tags.insert(tag.to_string().to_ascii_lowercase());
                } else {
                    input.set_value(tag);
                }
            }
            investment_info.set(info);
        })
    };

    let add_cost = {
        let investment_info = investment_info.clone();
        Callback::from(move |data: (String, f64)| {
            let mut info = (*investment_info).clone();
            info.costs.retain(|c| c.name != data.0);
            info.costs.push(InvestmentCost {
                name: data.0.clone(),
                value: data.1,
            });
            investment_info.set(info);
        })
    };

    let remove_cost = {
        let investment_info = investment_info.clone();
        Callback::from(move |name: String| {
            let mut info = (*investment_info).clone();
            info.costs.retain(|c| c.name != name);
            investment_info.set(info);
        })
    };

    let remove_tag = {
        let investment_info = investment_info.clone();
        Callback::from(move |name: String| {
            let mut info = (*investment_info).clone();
            info.tags.remove(&name);
            investment_info.set(info);
        })
    };

    let set_photos = {
        let investment_info = investment_info.clone();
        Callback::from(move |photos: Vec<String>| {
            let mut info = (*investment_info).clone();
            let mut p = photos.clone();
            info.photos.clear();
            info.photos.append(&mut p);
            investment_info.set(info);
        })
    };

    debug!("info: {:#?}", *investment_info);

    let mut sorted_tags = info.tags.iter().collect::<Vec<&String>>();
    sorted_tags.sort();

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
                        { for sorted_tags.iter().map(|t| html!(
                            <Tag remove={&remove_tag} name={<&std::string::String>::clone(t).clone()}></Tag>
                        ))}
                    </div>
                    <div class="input-group mb-3 input-group-sm">
                        <span class="input-group-text">{"Add tag"}</span>
                        <input type="text" class="form-control"
                            oninput={oninput_tags}/>
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
                                oninput={oninput_value}
                                />
                            <label for="investmentvalueGroup">{"Investment Value"}</label>
                        </div>
                    </div>
                    <Costs add={add_cost} remove={remove_cost}/>
                </fieldset>
                <Pictures set={set_photos} />
            </div>
            <div class="modal-footer">
                <button type="submit" class="btn btn-primary">{"Create"}</button>
            </div>
        </>
    )
}
