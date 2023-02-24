use super::costs::{Costs, InvestmentCost};
use super::pictures::Pictures;
use super::tag::Tag;
use std::collections::HashSet;
use std::rc::Rc;
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

pub enum InvestmentAction {
    AddPhoto(usize, String),
    SetName(String),
    SetMaturity(time::Date),
    SetExpiration(time::Date),
    SetDescription(String),
    AddTag(String),
    RemoveTag(String),
    SetValue(f64),
    AddCost(String, f64),
    RemoveCost(String),
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

impl Default for InvestmentInfo {
    fn default() -> Self {
        InvestmentInfo {
            name: "".to_string(),
            maturity: time::Date::MIN,
            expiration: time::Date::MIN,
            description: "".to_string(),
            tags: HashSet::new(),
            value: 0.0,
            costs: vec![],
            photos: vec![],
        }
    }
}

impl Reducible for InvestmentInfo {
    type Action = InvestmentAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut new = (*self).clone();
        match action {
            InvestmentAction::SetName(name) => {
                new.name = name;
            }
            InvestmentAction::SetDescription(description) => {
                new.description = description;
            }
            InvestmentAction::AddPhoto(index, path) => {
                if index + 1 > new.photos.len() {
                    new.photos.resize(index + 1, String::new());
                }
                new.photos[index] = path;
            }
            InvestmentAction::SetMaturity(date) => {
                new.maturity = date;
            }
            InvestmentAction::SetExpiration(date) => {
                new.expiration = date;
            }
            InvestmentAction::AddTag(tag) => {
                new.tags.insert(tag.to_ascii_lowercase());
            }
            InvestmentAction::RemoveTag(tag) => {
                new.tags.remove(&tag);
            }
            InvestmentAction::SetValue(value) => {
                new.value = value;
            }
            InvestmentAction::AddCost(name, value) => {
                new.costs.retain(|c| c.name != name);
                new.costs.push(InvestmentCost {
                    name: name.clone(),
                    value,
                });
            }
            InvestmentAction::RemoveCost(name) => {
                new.costs.retain(|c| c.name != name);
            }
        };
        new.into()
    }
}

#[function_component(ManageInvestment)]
pub fn manage_investment(_props: &Props) -> Html {
    let investment_info = use_reducer(InvestmentInfo::default);

    let info = (*investment_info).clone();
    let format = format_description!("[year]-[month]-[day]");

    let oninput_name = {
        let investment_info = investment_info.dispatcher();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            investment_info.dispatch(InvestmentAction::SetName(input.value()));
        })
    };

    let oninput_description = {
        let investment_info = investment_info.dispatcher();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            investment_info.dispatch(InvestmentAction::SetDescription(input.value()));
        })
    };

    let oninput_value = {
        let investment_info = investment_info.dispatcher();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if let Ok(v) = input.value().parse::<f64>() {
                investment_info.dispatch(InvestmentAction::SetValue(v));
            }
        })
    };

    let oninput_maturity = {
        let investment_info = investment_info.dispatcher();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if let Ok(d) = time::Date::parse(&input.value(), &format) {
                investment_info.dispatch(InvestmentAction::SetMaturity(d));
            };
        })
    };

    let oninput_expiration = {
        let investment_info = investment_info.dispatcher();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if let Ok(d) = time::Date::parse(&input.value(), &format) {
                investment_info.dispatch(InvestmentAction::SetExpiration(d));
            };
        })
    };

    let oninput_tags = {
        let investment_info = investment_info.dispatcher();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let current: String = input.value();
            let mut tags = current.split(',').map(str::trim).peekable();

            while let Some(tag) = tags.next() {
                if tags.peek().is_some() {
                    investment_info.dispatch(InvestmentAction::AddTag(tag.to_string()));
                } else {
                    input.set_value(tag);
                }
            }
        })
    };

    let add_cost = {
        let investment_info = investment_info.dispatcher();
        Callback::from(move |data: (String, f64)| {
            investment_info.dispatch(InvestmentAction::AddCost(data.0.clone(), data.1));
        })
    };

    let remove_cost = {
        let investment_info = investment_info.dispatcher();
        Callback::from(move |name: String| {
            investment_info.dispatch(InvestmentAction::RemoveCost(name));
        })
    };

    let remove_tag = {
        let investment_info = investment_info.dispatcher();
        Callback::from(move |name: String| {
            investment_info.dispatch(InvestmentAction::RemoveTag(name));
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
                <Pictures dispatcher={investment_info.dispatcher()} />
            </div>
            <div class="modal-footer">
                <button type="submit" class="btn btn-primary">{"Create"}</button>
            </div>
        </>
    )
}
