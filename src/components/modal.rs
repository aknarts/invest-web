use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Prop {
    pub close: Callback<MouseEvent>,
    pub active: bool,
    pub title: String,
    pub children: Children,
}

#[function_component(Modal)]
pub fn modal(props: &Prop) -> Html {
    let active_class = if props.active {
        (Some("show"), "display: block;")
    } else {
        (None, "display: none;")
    };

    html!(
        <div class={classes!("modal", "fade", active_class.0)} style={active_class.1}>
            <div class="modal-dialog modal-dialog-scrollable">
                <div class="modal-content">
                    <div class="modal-header">
                        <h1 class="modal-title fs-5" id="exampleModalLabel">{props.title.clone()}</h1>
                        <button type="button" class="btn-close" onclick={&props.close}></button>
                    </div>
                    { for props.children.iter() }
                </div>
            </div>
        </div>
    )
}
