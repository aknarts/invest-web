use crate::components::table::types::{ColumnBuilder, Table, TableData};
use crate::components::table::Options;
use crate::types::WrapCallback;
use serde::Serialize;
use serde_value::Value;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::use_counter;

#[derive(Clone, Debug, PartialEq)]
pub struct InvestmentCost {
    pub name: String,
    pub value: f64,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub add: Callback<(String, f64)>,
    pub remove: Callback<String>,
}

#[function_component(Costs)]
pub fn costs(props: &Props) -> Html {
    let costs: UseStateHandle<Vec<InvestmentCost>> = use_state(Vec::new);
    let updates = use_counter(0);
    let add = props.add.clone();
    let remove = props.remove.clone();
    let cost_info = use_state(|| InvestmentCost {
        name: String::new(),
        value: 0.0,
    });

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
            if let Ok(v) = input.value().parse::<f64>() {
                info.value = v;
            }
            cost_info.set(info);
        })
    };

    let add_cost = {
        let cost_info = cost_info.clone();
        let costs = costs.clone();
        let updates = updates.clone();
        let add = add.clone();
        Callback::from(move |_e: MouseEvent| {
            let mut info = (*cost_info).clone();
            let mut costs_info = (*costs).clone();
            costs_info.retain(|c| c.name != info.name);
            costs_info.push(InvestmentCost {
                name: info.name.clone(),
                value: info.value,
            });
            add.emit((info.name.clone(), info.value));
            info.name = String::new();
            info.value = 0.0;
            costs.set(costs_info);
            cost_info.set(info);

            updates.increase();
        })
    };
    let cost_value = format!("{}", cost_info.value);

    let remove_cost = {
        let costs = costs.clone();
        let updates = updates.clone();
        let remove = remove.clone();
        Callback::from(move |name: String| {
            let mut info = (*costs).clone();
            info.retain(|c| c.name != name);
            remove.emit(name);
            costs.set(info);
            updates.increase();
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
    for cost in (*costs).clone() {
        data.push(CostLine {
            name: cost.name.clone(),
            value: cost.value,
            remove: WrapCallback(Some(remove_cost.clone())),
        });
    }

    let options = Options {
        unordered_class: Some("fa-sort".to_string()),
        ascending_class: Some("fa-sort-up".to_string()),
        descending_class: Some("fa-sort-down".to_string()),
        orderable_classes: vec!["mx-1".to_string(), "fa-solid".to_string()],
    };

    let key = *updates;

    html!(
        <>
            <div class="h7">
                {"Costs"}
            </div>
            <div class="input-group mb-2">
                <Table<CostLine> {options} classes={classes!("table", "table-hover")} columns={columns} data={data} orderable={true} key={key}/>
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
        </>
    )
}

#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Serialize)]
struct CostLine {
    name: String,
    value: f64,
    remove: WrapCallback,
}

impl TableData for CostLine {
    fn get_field_as_html(&self, field_name: &str) -> crate::components::table::error::Result<Html> {
        let html = match field_name {
            "name" => html!({ &self.name }),
            "value" => html!({ format!("{:.2}", &self.value) }),
            "actions" => {
                let onclick_delete = {
                    let name = self.name.clone();
                    let remove = self.remove.0.as_ref().unwrap().clone();
                    Callback::from(move |_| {
                        remove.emit(name.clone());
                    })
                };

                html!(<button type="button" onclick={onclick_delete} class="btn btn-danger float-end">{"-"}</button>)
            }
            &_ => {
                html!()
            }
        };
        Ok(html)
    }

    fn get_field_as_value(
        &self,
        field_name: &str,
    ) -> crate::components::table::error::Result<Value> {
        let value = match field_name {
            "name" => serde_value::to_value(&self.name),
            "value" => serde_value::to_value(self.value),
            &_ => serde_value::to_value(""),
        };
        Ok(value.unwrap())
    }

    fn matches_search(&self, _needle: Option<String>) -> bool {
        true
    }
}
