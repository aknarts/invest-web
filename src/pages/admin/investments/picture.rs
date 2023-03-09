use crate::pages::admin::investments::modal::{InvestmentAction, InvestmentInfo};
use crate::pages::admin::investments::pictures::{Actions, Data, Reducer};
use crate::services::admin::upload_picture;
use base64::engine::general_purpose;
use base64::Engine;
use tracing::{debug, error, warn};
use uuid::Uuid;
use yew::platform::spawn_local;
use yew::prelude::*;
use yew::{html, Html};
use yew_hooks::use_counter;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub data: Data,
    pub uploads_dispatcher: UseReducerDispatcher<InvestmentInfo>,
    pub pictures_dispatcher: UseReducerDispatcher<Reducer>,
}

#[function_component(Picture)]
pub fn picture(props: &Props) -> Html {
    let name = props.data.name.clone();
    let data = props.data.clone();
    let id = data.id;
    let index = use_context::<usize>().unwrap();
    let uploads_dispatcher = props.uploads_dispatcher.clone();
    let pictures_dispatcher = props.pictures_dispatcher.clone();
    let bytes = data.bytes.clone();
    let path = data.path.clone();
    let id_state = use_state(Uuid::new_v4);
    let being_dragged = use_state(|| false);
    let drag_over = use_counter(0);

    let id_from_state = *id_state;
    if id_from_state.ne(&id) {
        if let Some(p) = &path {
            uploads_dispatcher.dispatch(InvestmentAction::AddPhoto(index, p.clone()));
        }

        id_state.set(id);
    };

    if bytes.is_some() && data.path.is_none() && !props.data.started_upload {
        let b = bytes.clone();
        let mime = data.mime.clone();
        let name = name.clone();
        let id = data.id;
        pictures_dispatcher.dispatch(Actions::UploadStarted(id));
        let photos_dispatcher = pictures_dispatcher.clone();
        spawn_local(async move {
            let multipart = match b {
                None => {
                    return;
                }
                Some(data) => {
                    photos_dispatcher.dispatch(Actions::Loaded(id, data.clone()));
                    debug!("Starting image upload");
                    let multipart = reqwest::multipart::Form::new();
                    let mut file = reqwest::multipart::Part::bytes(data);
                    file = file.file_name(name.clone());
                    file = match file.mime_str(&mime) {
                        Err(e) => {
                            error!("Unable to set mime type: {e}");
                            return;
                        }
                        Ok(p) => p,
                    };
                    multipart.part(name.clone(), file)
                }
            };

            match upload_picture(multipart).await {
                Ok(upload) => {
                    if let Some(path) = &upload.path {
                        photos_dispatcher.dispatch(Actions::Uploaded(id, path.clone()));
                    };
                }
                Err(_e) => {}
            };
        });
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
                if let Err(e) = input.set_data("text/id", &format!("{id}")) {
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

    let on_drop = {
        let id = index;
        let drag_over = drag_over.clone();
        let pictures_dispatch = pictures_dispatcher;
        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            drag_over.set(0);
            if let Some(input) = e.data_transfer() {
                if let Ok(value) = input.get_data("text/id") {
                    if let Ok(int) = value.parse::<usize>() {
                        debug!("Dropped {} on {}", int, id);
                        pictures_dispatch.dispatch(Actions::Move(int, id));
                    };
                }
            };
        })
    };

    let on_drag_end = {
        let being_dragged = being_dragged.clone();
        Callback::from(move |_e: DragEvent| {
            being_dragged.set(false);
        })
    };

    let b = &(bytes);
    let p = path.clone();
    let thumb = path;

    let u = p.is_some();

    let dragged = *being_dragged;

    let drag_class = if dragged { Some("opacity-50") } else { None };

    let drag_over_class = if dragged {
        None
    } else if (*drag_over) > 0 {
        Some("text-bg-secondary")
    } else {
        None
    };

    html!(
        <div class={classes!("mb-3", drag_class)}
                ondrop={on_drop}
                ondragover={|e: DragEvent| e.prevent_default() }
                ondragenter={on_drag_over}
                ondragleave={on_drag_leave}>
            <div ref={element}
                class={classes!("card", "w-100", drag_over_class)}
                ondragstart={on_drag_start}
                ondragend={on_drag_end}
                draggable="true">
                <div class="row no-gutters">
                    <div class="col-md-4 d-flex align-items-center">
                        {
                            b.as_ref().map_or(html!(<div class="spinner-border m-auto" role="status">
                                      <span class="sr-only">{"Loading..."}</span>
                                    </div>), |img| html!(
                            <div class="container card-img">
                                <div class ="row position-relative">
                                    <img class="col" src={ thumb.map_or_else(
                                            || {
                                                format!(
                                                    "data:{};base64,{}",
                                                    data.mime,
                                                    general_purpose::STANDARD.encode(img)
                                                )
                                            },
                                            |p| {
                                                format!("http://127.0.0.1:8081/{}", p.replace(".jpg", "_thumb.jpg"))

                                            },
                                        ) } style="max-width:100%; max-height:100%;"/>
                                    if !u {
                                        <div class="col position-absolute translate-middle top-50 start-50 spinner-border m-auto" role="status">
                                            <span class="sr-only">{"Loading..."}</span>
                                        </div>
                                    }
                                </div>
                            </div>
                            ))

                        }
                    </div>
                    <div class="col-md-8">
                        <div class="card-body">
                            <h5 class="card-title">{name}<br /><small class="card-title text-muted">{data.mime}</small></h5>
                            { p.map(|path| { html!(<>{"url: "}{path}</>)}) }
                        </div>
                    </div>
                </div>
            </div>
        </div>
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

        if item.type_().ne("text/id") {
            return None;
        }
    }

    Some(1)
}
