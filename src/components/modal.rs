use tracing::debug;
use yew::prelude::*;
use yew_hooks::use_click_away;

#[derive(Properties, PartialEq)]
pub struct Prop {
    pub close: Callback<MouseEvent>,
    pub active: bool,
    pub title: String,
    pub children: Children,
    #[prop_or(true)]
    pub autoclose: bool,
}

#[function_component(Modal)]
pub fn modal(props: &Prop) -> Html {
    let active_state = use_state(|| false);
    let node = use_node_ref();
    let active = *active_state;
    let active_class = if props.active {
        if !active {
            active_state.set(true);
        };
        (Some("show"), "display: block;")
    } else {
        if active {
            active_state.set(false);
        };
        (None, "display: none;")
    };
    debug!("Active: {}", *active_state);
    // TODO: DOES NOT WORK
    {
        let autoclose = props.autoclose;
        let close = props.close.clone();
        let active_state = active_state.clone();
        use_click_away(node.clone(), move |_: Event| {
            debug!("Active: {}", *active_state);
            if *active_state && autoclose {
                debug!("Closing!");
                close.emit(MouseEvent::new("mousedown").unwrap());
            }
        });
    }

    html!(
        <div ref={node} class={classes!("modal", "fade", active_class.0)} style={active_class.1}>
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
