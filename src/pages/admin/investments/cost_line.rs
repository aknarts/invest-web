use crate::pages::admin::investments::costs::InvestmentCost;
use crate::pages::admin::investments::modal::{InvestmentAction, InvestmentInfo};
use tracing::{debug, warn};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::{html, Html};
use yew_hooks::use_counter;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub data: InvestmentCost,
    pub callback: UseReducerDispatcher<InvestmentInfo>,
}

#[function_component(CostLine)]
pub fn cost_line(props: &Props) -> Html {
    let cost_info = use_state(|| InvestmentCost {
        name: String::new(),
        value: 0.0,
    });
    let editing = use_state(|| false);
    let name = props.data.name.clone();
    let value = props.data.value;
    let index = use_context::<usize>().unwrap();
    let being_dragged = use_state(|| false);
    let drag_over = use_counter(0);
    let name_state = use_state(String::new);
    let name_from_state = (*name_state).clone();
    if name_from_state.ne(&name) {
        editing.set(false);
        name_state.set(name.clone());
    }

    let remove_cost = {
        let dispatcher = props.callback.clone();
        Callback::from(move |_e: MouseEvent| {
            dispatcher.dispatch(InvestmentAction::RemoveCost(index));
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
            if let Ok(v) = input.value().parse::<f64>() {
                info.value = v;
            }
            cost_info.set(info);
        })
    };

    let edit_init = {
        let editing = editing.clone();
        let name = name.clone();
        let value = value;
        let cost_info = cost_info.clone();
        Callback::from(move |_e: MouseEvent| {
            editing.set(true);
            cost_info.set(InvestmentCost {
                name: name.clone(),
                value,
            });
        })
    };

    let edit_cancel = {
        let editing = editing.clone();
        Callback::from(move |_e: MouseEvent| {
            editing.set(false);
        })
    };

    let edit_confirm = {
        let dispatcher = props.callback.clone();
        let cost_info = cost_info.clone();
        let editing = editing.clone();
        Callback::from(move |_e: MouseEvent| {
            editing.set(false);
            let info = (*cost_info).clone();
            dispatcher.dispatch(InvestmentAction::EditCost(
                index,
                info.name.clone(),
                info.value,
            ));
        })
    };

    let element = use_node_ref();

    let on_drag_start = {
        let being_dragged = being_dragged.clone();
        let id = index;
        let element = element.clone();
        Callback::from(move |e: DragEvent| {
            if let Some(input) = e.data_transfer() {
                if let Err(e) = input.clear_data() {
                    warn!("Unable to clear drag data: {:?}", e);
                };
                if let Err(e) = input.set_data("text/cost", &format!("{id}")) {
                    warn!("Unable to set drag data: {:?}", e);
                };
                input.set_effect_allowed("move");
                input.set_drop_effect("move");
                if let Some(el) = element.get() {
                    if let Some(par) = el.parent_element() {
                        input.set_drag_image(&par, 0, 0);
                    }
                }

                being_dragged.set(true);
            }
        })
    };

    let on_drag_leave = {
        let drag_over = drag_over.clone();
        Callback::from(move |e: DragEvent| {
            if let Some(input) = e.data_transfer() {
                if validate_list(&input.items()).is_none() {
                    return;
                }
                drag_over.decrease();
            };
        })
    };

    let on_drag_over = {
        let drag_over = drag_over.clone();
        Callback::from(move |e: DragEvent| {
            if let Some(input) = e.data_transfer() {
                if validate_list(&input.items()).is_none() {
                    return;
                }
                drag_over.increase();
            };
        })
    };

    let on_drag_end = {
        let being_dragged = being_dragged.clone();
        Callback::from(move |_e: DragEvent| {
            being_dragged.set(false);
        })
    };

    let on_drop = {
        let id = index;
        let editing = editing.clone();
        let dispatcher = props.callback.clone();
        let drag_over = drag_over.clone();
        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            drag_over.set(0);
            if let Some(input) = e.data_transfer() {
                if let Ok(value) = input.get_data("text/cost") {
                    if let Ok(int) = value.parse::<usize>() {
                        debug!("Dropped {} on {}", int, id);
                        dispatcher.dispatch(InvestmentAction::MoveCost(int, id));
                        editing.set(false);
                    };
                }
            };
        })
    };

    let dragged = *being_dragged;

    let drag_class = if dragged { Some("opacity-50") } else { None };

    let drag_over_class = if dragged {
        None
    } else if (*drag_over) > 0 {
        Some("text-bg-secondary")
    } else {
        None
    };

    let edit = *editing;
    let cost_value = format!("{}", cost_info.value);

    html!(
        <tr ref={element}
            ondrop={on_drop}
            ondragstart={on_drag_start}
            ondragend={on_drag_end}
            ondragover={|e: DragEvent| e.prevent_default() }
            ondragenter={on_drag_over}
            ondragleave={on_drag_leave}
            class={classes!(drag_over_class, drag_class)}
            draggable={format!("{}", !edit)}>
            <td scope="row">
                if edit {
                    <input
                        class="form-control"
                        type="text"
                        placeholder="Cost Name"
                        oninput={oninput_cost_name}
                        value={cost_info.name.clone()}
                        />
                } else {
                    {name}
                }
            </td>
            <td>
                if edit {
                    <input
                        class="form-control"
                        type="number"
                        placeholder="0"
                        oninput={oninput_cost_value}
                        value={cost_value}
                        />
                } else {
                    {value}
                }
            </td>
            <td>
                if edit {
                    <button type="button" class="btn btn-success" onclick={edit_confirm}><i class="fa-solid fa-check"></i></button>
                    <button type="button" class="btn btn-danger" onclick={edit_cancel}><i class="fa-solid fa-ban"></i></button>
                } else {
                    <button type="button" class="btn btn-info" onclick={edit_init}><i class="fa-regular fa-pen-to-square"></i></button>
                    <button type="button" class="btn btn-danger" onclick={remove_cost}><i class="fa-regular fa-trash-can"></i></button>
                }
            </td>
        </tr>
    )
}

fn validate_list(list: &web_sys::DataTransferItemList) -> Option<i32> {
    if list.length() != 1 {
        return None;
    }

    if let Some(item) = list.get(0) {
        if item.kind().ne("string") {
            return None;
        };

        if item.type_().ne("text/cost") {
            return None;
        }
    }

    Some(1)
}
