use crate::error;
use crate::pages::admin::investments::modal::PictureInfo;
use crate::services::admin::upload_picture;
use base64::engine::general_purpose;
use base64::Engine;
use std::borrow::Borrow;
use tracing::{debug, error};
use yew::prelude::*;
use yew::{html, Html};
use yew_hooks::use_async;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
    pub position: usize,
    pub data: PictureInfo,
    pub bytes: Vec<u8>,
    pub loaded: Callback<(String, Vec<u8>)>,
}

#[function_component(Picture)]
pub fn picture(props: &Props) -> Html {
    let name = props.name.clone();
    let data = props.data.clone();
    let position = props.position.clone();

    let reader = use_state(|| None);
    let uploaded = use_state(|| false);

    let upload_images = {
        let b = data.data.clone();
        let mime = data.mime.clone();
        let name = name.clone();
        use_async(async move {
            let multipart = if !b.is_empty() {
                let multipart = reqwest::multipart::Form::new();
                let mut file = reqwest::multipart::Part::bytes(b);
                file = file.file_name(name.clone());
                file = match file.mime_str(&mime) {
                    Err(e) => {
                        error!("Unable to set mime type: {e}");
                        return Err(error::Error::BadRequest);
                    }
                    Ok(p) => p,
                };
                multipart.part(name.clone(), file)
            } else {
                return Err(error::Error::BadRequest);
            };

            upload_picture(multipart).await
        })
    };

    if reader.is_none() {
        let task = {
            let bytes = data.data.clone();
            let loaded = props.loaded.borrow();
            let name = name.clone();
            gloo::file::callbacks::read_as_bytes(&data.picture, move |res| match res {
                Ok(contents) => {
                    debug!("Finished loading file");
                    loaded.emit((name, contents));
                    // upload_images.run();
                }
                Err(e) => {
                    error!("Unable to read file: {:?}", e);
                }
            })
        };
        reader.set(Some(task));
    }

    let b = &data.data;

    html!(
        <div class="card mb-3" style="max-width: 100%;" draggable="true">
            <div class="row no-gutters">
                <div class="col-md-4 d-flex align-items-center">
                    {
                        if b.is_empty() {
                            html!(<div class="spinner-border m-auto" role="status">
                                  <span class="sr-only">{"Loading..."}</span>
                                </div>)
                        } else { html!(<>
                                <img class="card-img" src={format!("data:{};base64,{}",data.mime, general_purpose::STANDARD.encode(&b))} style="max-width:100%; max-height:100%;"/>
                                <div class="card-img position-absolute top-0 start-0 spinner-border m-auto" role="status">
                                    <span class="sr-only">{"Loading..."}</span>
                                </div>
                            </>)
                    }

                    }
                </div>
                <div class="col-md-8">
                    <div class="card-body">
                        <h5 class="card-title"><span class="font-weight-bold">{format!("#{} ", position)}</span>{name}<br /><small class="card-title text-muted">{data.mime}</small></h5>
                    </div>
                </div>
            </div>
        </div>
    )
}
