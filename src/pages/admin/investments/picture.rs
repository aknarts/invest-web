use super::pictures::PictureInfo;
use crate::error;
use crate::services::admin::upload_picture;
use base64::engine::general_purpose;
use base64::Engine;
use tracing::{debug, error, warn};
use yew::prelude::*;
use yew::{html, Html};
use yew_hooks::use_async;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub data: PictureInfo,
}

#[function_component(Picture)]
pub fn picture(props: &Props) -> Html {
    let name = props.data.name.clone();
    let data = props.data.clone();
    let index = use_context::<usize>().unwrap();
    let uploaded = use_state(|| false);
    let bytes = use_state(|| None::<Vec<u8>>);
    let reader = use_state(|| None);
    let path = use_state(|| None);
    let error = use_state(|| None);
    let being_dragged = use_state(|| false);
    let drag_over = use_state(|| false);

    let upload_images = {
        let b = (*bytes).clone();
        let mime = data.mime.clone();
        let name = name.clone();
        use_async(async move {
            let multipart = match b {
                None => {
                    return Err(error::Error::BadRequest);
                }
                Some(data) => {
                    let multipart = reqwest::multipart::Form::new();
                    let mut file = reqwest::multipart::Part::bytes(data);
                    file = file.file_name(name.clone());
                    file = match file.mime_str(&mime) {
                        Err(e) => {
                            error!("Unable to set mime type: {e}");
                            return Err(error::Error::BadRequest);
                        }
                        Ok(p) => p,
                    };
                    multipart.part(name.clone(), file)
                }
            };

            upload_picture(multipart).await
        })
    };

    {
        let p = path.clone();
        let e = error.clone();
        let uploaded = uploaded.clone();
        use_effect_with_deps(
            move |upload_images| {
                upload_images.data.as_ref().map_or_else(
                    || {},
                    |upload| {
                        e.set(upload.error.clone());
                        p.set(upload.path.clone());
                        uploaded.set(true);
                    },
                );
            },
            upload_images.clone(),
        );
    }

    if reader.is_none() {
        let task = {
            let bytes = bytes.clone();
            gloo::file::callbacks::read_as_bytes(&data.picture, move |res| match res {
                Ok(contents) => {
                    debug!("Finished loading file");
                    bytes.set(Some(contents));
                    upload_images.run();
                }
                Err(e) => {
                    error!("Unable to read file: {:?}", e);
                }
            })
        };
        reader.set(Some(task));
    }

    let on_drag_over = {
        let drag_over = drag_over.clone();
        Callback::from(move |e: DragEvent| {
            if let Some(input) = e.data_transfer() {
                match input.get_data("text/plain") {
                    Ok(value) => {
                        debug!("Dragged over: {}", value);
                        drag_over.set(true);
                    }
                    Err(e) => {
                        warn!("unable to get data: {:?}", e);
                    }
                }
            };
        })
    };

    let on_drag_leave = {
        let drag_over = drag_over.clone();
        Callback::from(move |e: DragEvent| {
            if let Some(input) = e.data_transfer() {
                match input.get_data("text/plain") {
                    Ok(_) => {
                        drag_over.set(false);
                    }
                    Err(e) => {
                        warn!("unable to get data: {:?}", e);
                    }
                }
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
                debug!("Setting id: {}", id);
                if let Err(e) = input.set_data("text/plain", &format!("{id}")) {
                    debug!("Unable to set data: {:?}", e);
                };
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
        Callback::from(move |e: DragEvent| {
            debug!("Dropped on {}", id);
            if let Some(input) = e.data_transfer() {
                if let Ok(value) = input.get_data("text") {
                    debug!("Dropped {} on {}", value, id);
                }
            };
        })
    };

    let on_drag_end = {
        let being_dragged = being_dragged.clone();
        let id = index;
        Callback::from(move |_e: DragEvent| {
            debug!("Drag ended for {}", id);
            being_dragged.set(false);
        })
    };

    let b = &(*bytes);
    let p = (*path).clone();
    let e = (*error).clone();
    let u = *uploaded;

    let dragged = *being_dragged;

    let drag_class = if dragged { Some("opacity-50") } else { None };

    let drag_over_class = if dragged {
        None
    } else if *drag_over {
        Some("text-bg-secondary")
    } else {
        None
    };

    html!(
        <div class={classes!("mb-3", drag_class)}
                ondrop={on_drop}
                ondragover={on_drag_over}
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
                                    <img class="col" src={format!("data:{};base64,{}",data.mime, general_purpose::STANDARD.encode(img))} style="max-width:100%; max-height:100%;"/>
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
                            { e.map(|error| { html!(<>{"error: "}{error}</>)}) }
                        </div>
                    </div>
                </div>
            </div>
        </div>
    )
}
