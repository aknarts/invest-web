use super::costs::{Costs, InvestmentCost};
use super::pictures::Pictures;
use super::tags::Tags;
use crate::services::admin::create_investment;
use serde::Serialize;
use std::collections::HashSet;
use std::rc::Rc;
use time::macros::format_description;
use tracing::debug;
use web_sys::{HtmlInputElement, MouseEvent};
use yew::prelude::*;
use yew::{html, Callback, Html};
use yew_hooks::{use_async, UseCounterHandle};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub close: Callback<MouseEvent>,
    pub counter: UseCounterHandle,
}

pub enum InvestmentAction {
    Init,
    AddPhoto(usize, String, String),
    SetName(String),
    SetLocation(String),
    SetMaturity(Option<time::Date>),
    SetExpiration(Option<time::Date>),
    SetDescription(String),
    AddTag(String),
    RemoveTag(String),
    SetValue(f64),
    SetEarning(f64),
    AddCost(String, f64),
    RemoveCost(usize),
    EditCost(usize, String, f64),
    MoveCost(usize, usize),
}

#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct InvestmentInfo {
    pub name: String,
    pub location: String,
    pub maturity: Option<time::Date>,
    pub expiration: Option<time::Date>,
    pub description: String,
    pub tags: HashSet<String>,
    pub value: f64,
    pub earning: f64,
    pub costs: Vec<InvestmentCost>,
    pub photos: Vec<(String, String)>,
}

impl Default for InvestmentInfo {
    fn default() -> Self {
        Self {
            name: String::new(),
            location: String::new(),
            maturity: None,
            expiration: None,
            description: String::new(),
            tags: HashSet::new(),
            value: 0.0,
            earning: 0.0,
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
            InvestmentAction::Init => new = Self::default(),
            InvestmentAction::SetName(name) => new.name = name,
            InvestmentAction::SetDescription(description) => new.description = description,
            InvestmentAction::AddPhoto(index, path, desc) => {
                if index + 1 > new.photos.len() {
                    new.photos.resize(index + 1, (String::new(), String::new()));
                }
                new.photos[index] = (path, desc);
            }
            InvestmentAction::SetMaturity(date) => new.maturity = date,
            InvestmentAction::SetExpiration(date) => new.expiration = date,
            InvestmentAction::AddTag(tag) => {
                if !tag.is_empty() {
                    new.tags.insert(tag.to_ascii_lowercase());
                };
            }
            InvestmentAction::RemoveTag(tag) => {
                new.tags.remove(&tag);
            }
            InvestmentAction::SetValue(value) => new.value = value,
            InvestmentAction::SetEarning(value) => new.earning = value,
            InvestmentAction::AddCost(name, value) => {
                new.costs.push(InvestmentCost { name, value });
            }
            InvestmentAction::RemoveCost(index) => {
                new.costs.remove(index);
            }
            InvestmentAction::EditCost(index, name, value) => {
                new.costs.remove(index);
                new.costs.insert(index, InvestmentCost { name, value });
            }
            InvestmentAction::MoveCost(from, to) => {
                let temp = new.costs.remove(from);
                new.costs.insert(to, temp);
            }
            InvestmentAction::SetLocation(location) => new.location = location,
        };
        new.into()
    }
}

#[function_component(ManageInvestment)]
pub fn manage_investment(props: &Props) -> Html {
    let investment_info = use_reducer(InvestmentInfo::default);

    let info = (*investment_info).clone();
    let format = format_description!("[year]-[month]-[day]");

    let investment_create = {
        let investment_info = investment_info.clone();
        use_async(async move {
            let request = (*investment_info).clone();
            create_investment(request).await
        })
    };

    let onsubmit = {
        let investment = investment_info.dispatcher();
        let close = props.close.clone();
        let investment_create = investment_create;
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            investment_create.run();
            close.emit(MouseEvent::new("mousedown").unwrap());
            investment.dispatch(InvestmentAction::Init);
        })
    };

    let oninput_name = {
        let investment_info = investment_info.dispatcher();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            investment_info.dispatch(InvestmentAction::SetName(input.value()));
        })
    };

    let oninput_location = {
        let investment_info = investment_info.dispatcher();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            investment_info.dispatch(InvestmentAction::SetLocation(input.value()));
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

    let oninput_earnings = {
        let investment_info = investment_info.dispatcher();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if let Ok(v) = input.value().parse::<f64>() {
                investment_info.dispatch(InvestmentAction::SetEarning(v));
            }
        })
    };

    let oninput_maturity = {
        let investment_info = investment_info.dispatcher();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if let Ok(d) = time::Date::parse(&input.value(), &format) {
                investment_info.dispatch(InvestmentAction::SetMaturity(Some(d)));
            };
        })
    };

    let oninput_expiration = {
        let investment_info = investment_info.dispatcher();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if let Ok(d) = time::Date::parse(&input.value(), &format) {
                investment_info.dispatch(InvestmentAction::SetExpiration(Some(d)));
            };
        })
    };

    let dispatcher = investment_info.dispatcher();

    debug!("info: {:#?}", *investment_info);

    let mut sorted_tags = info
        .tags
        .iter()
        .map(std::clone::Clone::clone)
        .collect::<Vec<String>>();
    sorted_tags.sort();

    let data = (*investment_info).clone();
    let value = format!("{}", data.value);
    let earning = format!("{}", data.earning);
    let rate = (data.earning * 12.0) / data.value;
    let paearning = if value.eq("0") || earning.eq("0") {
        "0".to_string()
    } else {
        format!("{:.1}", (rate * 100.0))
    };
    let total_earnings = if data.maturity.is_none() || data.expiration.is_none() {
        None
    } else {
        #[allow(clippy::cast_precision_loss)]
        Some(format!(
            "{:.1}",
            (rate * (data.maturity.unwrap() - data.expiration.unwrap()).whole_days() as f64
                / 365.0)
                * 100.0
        ))
    };

    let exp = data.expiration.map(|d| d.to_string());
    let mat = data.maturity.map(|d| d.to_string());

    html!(
        <>
            <div class="modal-body">
                <span class="h5">
                    {"General"}
                </span>
                <div class="border-bottom">
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
                        <span class="input-group-text" title={"When will the investment round end?"}>
                          <i class="fa-regular fa-calendar-plus"></i>
                        </span>
                        <div class="form-floating">
                            <input
                                class="form-control"
                                type="date"
                                id="investmentexpirationGroup"
                                value={exp}
                                oninput={oninput_expiration}
                                />
                            <label for="investmentexpirationGroup">{"Investment Expiration"}</label>
                        </div>
                        <span class="input-group-text" title={"When will the investment be returned?"}>
                          <i class="fa-regular fa-calendar-check"></i>
                        </span>
                        <div class="form-floating">
                            <input
                                class="form-control"
                                type="date"
                                id="investmentmaturityGroup"
                                value={mat}
                                oninput={oninput_maturity}
                                />
                            <label for="investmentmaturityGroup">{"Investment Maturity"}</label>
                        </div>
                    </div>
                    <div class="input-group mb-2">
                        <span class="input-group-text">
                          <i class="fa-solid fa-map-location-dot"></i>
                        </span>
                        <div class="form-floating">
                            <input
                                class="form-control"
                                type="text"
                                id="investmentlocationGroup"
                                placeholder="Investment Location"
                                value={info.location.clone()}
                                oninput={oninput_location}
                                />
                            <label for="investmentlocationGroup">{"Investment Location"}</label>
                        </div>
                    </div>
                    <div class="input-group mb-2">
                        <div class="form-floating">
                          <textarea class="form-control" id="floatingTextarea2" style="height: 100px"
                                oninput={oninput_description} value={info.description.clone()}></textarea>
                          <label for="floatingTextarea2">{"Description"}</label>
                        </div>
                    </div>
                </div>
                <div class="border-bottom">
                    <Tags callback={dispatcher.clone()} tags={sorted_tags.clone()} />
                </div>
                <div class="border-bottom">
                    <span class="h5">
                        {"Financials"}
                    </span>
                    <div class="container">
                        <div class="row">
                            <div class="col">
                                <div class="input-group mb-2">
                                    <span class="input-group-text">
                                      <i class="fa-solid fa-house-chimney-medical" />
                                    </span>
                                    <div class="form-floating">
                                        <input
                                            class="form-control"
                                            type="number"
                                            id="investmentvalueGroup"
                                            placeholder="0"
                                            oninput={oninput_value}
                                            value={value}
                                            />
                                        <label for="investmentvalueGroup">{"Investment Value"}</label>
                                    </div>
                                </div>
                                <div class="input-group mb-2">
                                    <span class="input-group-text">
                                      <i class="fa-solid fa-piggy-bank" />
                                    </span>
                                    <div class="form-floating">
                                        <input
                                            class="form-control"
                                            type="number"
                                            id="investmentearningGroup"
                                            placeholder="0"
                                            oninput={oninput_earnings}
                                            value={earning}
                                            />
                                        <label for="investmentearningGroup">{"Investment Earnings"}</label>
                                    </div>
                                </div>
                            </div>
                            <div class="col-md-auto text-center d-flex border">
                                <div class="align-self-center">
                                    <div>
                                        <span class="h1">{paearning}</span>
                                        <span class="h3">{"%"}</span>
                                        <span class="h5">{" pa"}</span>
                                    </div>
                                    { total_earnings.map(|total| html!(<div>
                                        <span class="h6">{total}{"% total"}</span>
                                    </div>))
                                    }

                                </div>
                            </div>
                        </div>
                    </div>
                </div>
                <div class="border-bottom">
                    <Costs costs={info.costs.clone()} callback={dispatcher}/>
                </div>
                <Pictures dispatcher={investment_info.dispatcher()} />
            </div>
            <div class="modal-footer">
                <form {onsubmit}>
                    <button type="submit" class="btn btn-primary">{"Create"}</button>
                </form>
            </div>
        </>
    )
}
