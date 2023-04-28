use crate::pages::admin::investments::modal::{InvestmentAction, InvestmentInfo};
use crate::pages::admin::investments::tag::Tag;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::{html, Html};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub callback: UseReducerDispatcher<InvestmentInfo>,
    pub tags: Vec<String>,
}

#[function_component(Tags)]
pub fn tags(props: &Props) -> Html {
    let callback = props.callback.clone();
    let tags = props.tags.clone();

    let oninput_tags = {
        let investment_info = callback.clone();
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

    html!(<>
            <div class="h7">{"Tags"}</div>
            <div class="container-fluid p-2">
                { for tags.iter().map(|t| html!(
                    <Tag callback={callback.clone()} name={<std::string::String>::clone(t).clone()}></Tag>
                ))}
            </div>
            <div class="input-group mb-3 input-group-sm">
                <span class="input-group-text" title={"Insert comma separated values"}>{"Add tag"}</span>
                <input type="text" class="form-control"
                    oninput={oninput_tags}/>
            </div>
        </>
    )
}
