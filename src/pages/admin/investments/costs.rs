use crate::pages::admin::investments::cost_line::CostLine;
use crate::pages::admin::investments::modal::{InvestmentAction, InvestmentInfo};
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct InvestmentCost {
    pub name: String,
    pub value: f64,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub costs: Vec<InvestmentCost>,
    pub callback: UseReducerDispatcher<InvestmentInfo>,
}

#[function_component(Costs)]
pub fn costs(props: &Props) -> Html {
    let costs = props.costs.clone();
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
        let dispatcher = props.callback.clone();
        Callback::from(move |_e: MouseEvent| {
            let mut info = (*cost_info).clone();
            dispatcher.dispatch(InvestmentAction::AddCost(info.name.clone(), info.value));
            info.name = String::new();
            info.value = 0.0;
            cost_info.set(info);
        })
    };
    let cost_value = format!("{}", cost_info.value);
    let dispatcher = props.callback.clone();

    html!(
        <>
            <div class="h7">
                {"Costs"}
            </div>
            <table class="table table-sm align-middle">
                <thead>
                    <tr>
                        <th scope="col">{ "Name" }</th>
                        <th scope="col">{ "Value" }</th>
                        <th scope="col"></th>
                    </tr>
                </thead>
                <tbody>
                    { for costs.iter().enumerate().map(|(i, n)| {
                        html!(
                            <ContextProvider<usize> context={i}>
                                <CostLine callback={dispatcher.clone()} data={n.clone()} />
                            </ContextProvider<usize>>
                            )
                        })
                    }
                </tbody>
                <tfoot>
                    <tr>
                        <td scope="row">
                            <input
                                class="form-control"
                                type="text"
                                placeholder="Cost Name"
                                oninput={oninput_cost_name}
                                value={cost_info.name.clone()}
                                />
                        </td>
                        <td>
                            <input
                                class="form-control"
                                type="number"
                                placeholder="0"
                                oninput={oninput_cost_value}
                                value={cost_value}
                                />
                        </td>
                        <td>
                            <button type="button" class="btn btn-success" onclick={add_cost}><i class="fa-solid fa-plus"></i></button>
                        </td>
                    </tr>
                </tfoot>
            </table>
        </>
    )
}

// #[derive(Default, Debug, Clone, PartialEq, PartialOrd, Serialize)]
// struct CostLine {
//     name: String,
//     value: f64,
//     remove: WrapCallback,
// }
//
// impl TableData for CostLine {
//     fn get_field_as_html(&self, field_name: &str) -> crate::components::table::error::Result<Html> {
//         let html = match field_name {
//             "name" => html!({ &self.name }),
//             "value" => html!({ format!("{:.2}", &self.value) }),
//             "actions" => {
//                 let onclick_delete = {
//                     let name = self.name.clone();
//                     let remove = self.remove.0.as_ref().unwrap().clone();
//                     Callback::from(move |_| {
//                         remove.emit(name.clone());
//                     })
//                 };
//
//                 html!(<button type="button" onclick={onclick_delete} class="btn btn-danger float-end">{"-"}</button>)
//             }
//             &_ => {
//                 html!()
//             }
//         };
//         Ok(html)
//     }
//
//     fn get_field_as_value(
//         &self,
//         field_name: &str,
//     ) -> crate::components::table::error::Result<Value> {
//         let value = match field_name {
//             "name" => serde_value::to_value(&self.name),
//             "value" => serde_value::to_value(self.value),
//             &_ => serde_value::to_value(""),
//         };
//         Ok(value.unwrap())
//     }
//
//     fn matches_search(&self, _needle: Option<String>) -> bool {
//         true
//     }
// }
