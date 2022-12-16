use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Prop {
    pub close: Callback<MouseEvent>,
    pub active: UseStateHandle<bool>,
    pub title: String,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Modal)]
pub fn modal(props: &Prop) -> Html {
    let node = use_node_ref();
    let active = *props.active;
    let active_class = if active {
        (Some("show"), "display: block;")
    } else {
        (None, "display: none;")
    };
    let title = Html::from_html_unchecked(AttrValue::from(format!(
        "<h1 class=\"modal-title fs-5\" id=\"exampleModalLabel\">{}</h1>",
        props.title.clone()
    )));

    html!(
        <div ref={node} class={classes!("modal", "fade", active_class.0)} style={active_class.1}>
            <div class="modal-dialog modal-dialog-scrollable">
                <div class="modal-content">
                    <div class="modal-header">
                        {title}
                        <button type="button" class="btn-close" onclick={&props.close}></button>
                    </div>
                    { for props.children.iter() }
                </div>
            </div>
        </div>
    )
}
