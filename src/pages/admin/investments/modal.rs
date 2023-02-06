use std::collections::HashSet;
use std::convert::Infallible;
use serde_value::Value;
use web_sys::{HtmlInputElement, MouseEvent};
use yew::prelude::*;
use yew::{html, Callback, Html};
use yew_hooks::UseCounterHandle;
use time::macros::format_description;
use serde::Serialize;
use tracing::debug;
use crate::components::table::Options;
use crate::components::table::types::{ColumnBuilder, Table, TableData};
use crate::pages::admin::investments::tag::Tag;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub close: Callback<MouseEvent>,
    pub counter: UseCounterHandle,
}

#[derive(Clone, Debug, PartialEq)]
pub struct InvestmentCost {
    pub name: String,
    pub value: f64,
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

    let cost_info = use_state(|| InvestmentCost {
        name: String::new(),
        value: 0.0,
    });
    let info = (*investment_info).clone();
    let cost = (*cost_info).clone();
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

    let oninput_cost_name = {
        let cost_info = cost_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*cost_info).clone();
            info.name = input.value();
            cost_info.set(info);
        })
    };

    let oninput_cost_value = {
        let cost_info = cost_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*cost_info).clone();
            if let Ok(v) = input.value().parse::<f64>() { info.value = v; }
            cost_info.set(info);
        })
    };

    let add_cost = {
        let cost_info = cost_info.clone();
        let investment_info = investment_info.clone();
        Callback::from(move |_| {
            let mut info = (*cost_info).clone();
            let mut invest_info = (*investment_info).clone();
            invest_info.costs.push(InvestmentCost { name: info.name.clone(), value: info.value });
            debug!("Costs: {:?}", invest_info.costs);
            info.name = String::new();
            info.value = 0.0;
            investment_info.set(invest_info);
            cost_info.set(info);
        })
    };
    let cost_value = format!("{}", cost_info.value);

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
            if let Ok(v) = input.value().parse::<f64>() { info.value = v; }
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

    let oninput_tags = {
        let investment_info = investment_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*investment_info).clone();
            let current: String = input.value();
            let mut tags = current.split(",").map(|t| t.trim()).peekable();

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

    let remove_tag = {
        let investment_info = investment_info.clone();
        Callback::from(move |name: String| {
            let mut info = (*investment_info).clone();
            info.tags.remove(&name);
            investment_info.set(info);
        })
    };


    let columns = vec![
        ColumnBuilder::new("name")
            .orderable(true)
            .data_property("name")
            .short_name("Cost Name")
            .header_class("user-select-none")
            .build(),
        ColumnBuilder::new("value")
            .orderable(true)
            .short_name("Value")
            .data_property("value")
            .header_class("user-select-none")
            .build(),
        ColumnBuilder::new("Actions")
            .data_property("actions")
            .short_name("")
            .header_class("user-select-none")
            .build(),
    ];
    let mut data = Vec::new();
    for cost in investment_info.costs.clone() {
        debug!("Pushing cost: {:?}", cost);
        data.push(CostLine { name: cost.name.clone(), value: cost.value })
    };
    debug!("Table data: {:?}", data);
    let options = Options {
        unordered_class: Some("fa-sort".to_string()),
        ascending_class: Some("fa-sort-up".to_string()),
        descending_class: Some("fa-sort-down".to_string()),
        orderable_classes: vec!["mx-1".to_string(), "fa-solid".to_string()],
    };

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
                            <Tag remove={&remove_tag} name={t.clone().clone()}></Tag>
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
                    <div class="h7">
                        {"Costs"}
                    </div>
                    <div class="input-group mb-2">
                        <Table<CostLine> {options} classes={classes!("table", "table-hover")} columns={columns} data={data} orderable={true}/>
                        <div class="input-group-text">
                            <input
                                type="text"
                                placeholder="Cost Name"
                                oninput={oninput_cost_name}
                                value={cost_info.name.clone()}
                                />
                        </div>
                        <input
                            class="form-control"
                            type="number"
                            placeholder="0"
                            oninput={oninput_cost_value}
                            value={cost_value}
                            />
                        <button type="button" class="btn btn-success" onclick={add_cost}>{"+"}</button>
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

#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Serialize)]
struct CostLine {
    name: String,
    value: f64,
}

impl TableData for CostLine {
    fn get_field_as_html(&self, field_name: &str) -> crate::components::table::error::Result<Html> {
        debug!("Getting: {field_name}");
        let html = match field_name {
            "name" => html!({ &self.name }),
            "value" => html!({ format!("{:.2}", &self.value) }),
            &_ => {
                html!()
            }
        };
        Ok(html)
    }

    fn get_field_as_value(&self, field_name: &str) -> crate::components::table::error::Result<Value> {
        debug!("Getting value: {field_name}");
        let value = match field_name {
            "name" => serde_value::to_value(&self.name),
            "value" => serde_value::to_value(&self.value),
            &_ => serde_value::to_value(""),
        };
        Ok(value.unwrap())
    }

    fn matches_search(&self, needle: Option<String>) -> bool {
        debug!("Testing search");
        true
    }
}