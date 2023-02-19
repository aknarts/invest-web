use super::pictures::PictureInfo;
use crate::error;
use crate::services::admin::upload_picture;
use base64::engine::general_purpose;
use base64::Engine;
use tracing::{debug, error};
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
    let bytes = use_state(|| None::<Vec<u8>>);
    let reader = use_state(|| None);
    let uploaded = use_state(|| false);

    let upload_images = {
        let b = data.bytes.clone();
        let mime = data.mime.clone();
        let name = name.clone();
        use_async(async move {
            let multipart = if b.is_empty() {
                return Err(error::Error::BadRequest);
            } else {
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
            };

            upload_picture(multipart).await
        })
    };

    if reader.is_none() {
        let task = {
            let bytes = bytes.clone();
            gloo::file::callbacks::read_as_bytes(&data.picture, move |res| match res {
                Ok(contents) => {
                    debug!("Finished loading file");
                    bytes.set(Some(contents));
                    // loaded.emit((name, contents));
                    // upload_images.run();
                }
                Err(e) => {
                    error!("Unable to read file: {:?}", e);
                }
            })
        };
        reader.set(Some(task));
    }

    let b = &(*bytes);

    html!(
        <div class="card mb-3" style="max-width: 100%;" draggable="true">
            <div class="row no-gutters">
                <div class="col-md-4 d-flex align-items-center">
                    {
                        b.as_ref().map_or(html!(<div class="spinner-border m-auto" role="status">
                                  <span class="sr-only">{"Loading..."}</span>
                                </div>), |img| html!(<>
                            <img class="card-img" src={format!("data:{};base64,{}",data.mime, general_purpose::STANDARD.encode(img))} style="max-width:100%; max-height:100%;"/>
                            <div class="card-img position-absolute top-0 start-0 spinner-border m-auto" role="status">
                                <span class="sr-only">{"Loading..."}</span>
                            </div>
                        </>
                        ))

                    }
                </div>
                <div class="col-md-8">
                    <div class="card-body">
                        <h5 class="card-title">{name}<br /><small class="card-title text-muted">{data.mime}</small></h5>
                    </div>
                </div>
            </div>
        </div>
    )
}
