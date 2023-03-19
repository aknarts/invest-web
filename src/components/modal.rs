use yew::prelude::*;

#[derive(Default, Eq, PartialEq)]
#[allow(dead_code)]
pub enum Size {
    Small,
    #[default]
    Medium,
    Large,
    XLarge,
    Fullscreen,
}

#[derive(Properties, PartialEq)]
pub struct Prop {
    pub close: Callback<MouseEvent>,
    pub active: UseStateHandle<bool>,
    pub title: String,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub size: Size,
    #[prop_or(true)]
    pub backdrop: bool,
    #[prop_or(true)]
    pub centered: bool,
    #[prop_or(true)]
    pub scrollable: bool,
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
        "<h1 class=\"modal-title fs-5\">{}</h1>",
        props.title.clone()
    )));

    let size = match props.size {
        Size::Small => Some("modal-sm"),
        Size::Medium => None,
        Size::Large => Some("modal-lg"),
        Size::XLarge => Some("modal-xl"),
        Size::Fullscreen => Some("modal-fullscreen"),
    };

    let centered = if props.centered {
        Some("modal-dialog-centered")
    } else {
        None
    };

    let scrollable = if props.scrollable {
        Some("modal-dialog-scrollable")
    } else {
        None
    };

    html!(
        <>
            <div ref={node} class={classes!("modal", "fade", active_class.0)} style={active_class.1}>
                <div class={classes!("modal-dialog", scrollable, size, centered)}>
                    <div class="modal-content">
                        <div class="modal-header">
                            {title}
                            <button type="button" class="btn-close" onclick={&props.close}></button>
                        </div>
                        { for props.children.iter() }
                    </div>
                </div>
            </div>
            if active && props.backdrop {
                <div class="modal-backdrop fade show"></div>
            }
        </>
    )
}
