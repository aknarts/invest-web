use crate::pages::admin::investments::picture::Picture;
use gloo::file::File;
use std::collections::HashMap;
use tracing::debug;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::{html, use_node_ref, use_state, Callback, Html, UseStateHandle};

#[derive(Clone, Debug, PartialEq)]
pub struct PictureInfo {
    pub name: String,
    pub mime: String,
    pub picture: File,
    pub bytes: Vec<u8>,
    pub order: usize,
}

#[derive(Properties, PartialEq)]
pub struct Props {}

#[function_component(Pictures)]
pub fn pictures(_props: &Props) -> Html {
    let pictures: UseStateHandle<HashMap<String, PictureInfo>> = use_state(HashMap::new);

    let image_loaded = {
        let pictures = pictures.clone();
        Callback::from(move |data: (String, Vec<u8>)| {
            let name = data.0;
            let mut bytes = data.1;
            let mut pict = (*pictures).clone();
            if let Some(pic) = pict.get(&name) {
                let mut new = pic.clone();
                new.bytes.append(&mut bytes);
                pict.insert(name, new);
            };
            pictures.set(pict);
            debug!("Image loaded");
        })
    };

    let on_image_select = {
        let pictures = pictures.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            process_pictures(&pictures, input.files());
        })
    };

    let on_image_drop = {
        let pictures = pictures.clone();

        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            if let Some(input) = e.data_transfer() {
                process_pictures(&pictures, input.files());
            };
        })
    };

    let file_picker = use_node_ref();
    let f_picker = file_picker.clone();
    let click_add_image = Callback::from(move |_| {
        if let Some(element) = f_picker.cast::<HtmlInputElement>() {
            element.click();
        };
    });

    let pics = (*pictures).clone();
    html!(<>
            <div class="h5">
                {"Pictures"}
            </div>

            <button type="button" class="btn btn-outline-secondary btn-lg px-5 py-5" style="width:100%" onclick={click_add_image}
                    ondrop={on_image_drop}
                    ondragover={Callback::from(|event: DragEvent| {
                        event.prevent_default();
                    })}
                    ondragenter={Callback::from(|event: DragEvent| {
                        event.prevent_default();
                    })}>{"Drag or Click"}</button>
            <input ref={file_picker} type="file" accept="image/jpeg" style="display:none;" onchange={on_image_select} multiple={true}/>
            { for pics.iter().map(|(name, data)|
                { html!(<Picture bytes={data.bytes.clone()} name={name.clone()} loaded={&image_loaded} position={data.order} data={data.clone()}></Picture>)}
            )}
        </>
    )
}

fn process_pictures(
    pictures: &UseStateHandle<HashMap<String, PictureInfo>>,
    input: Option<web_sys::FileList>,
) {
    let pic = load_files(input);
    let mut pict = (**pictures).clone();
    for p in pic {
        let name = p.name().clone();
        debug!("Processing: {name}");
        let mime = p.raw_mime_type().clone();
        let position = pict.len();
        pict.insert(
            name.clone(),
            PictureInfo {
                name: name.clone(),
                mime: mime.clone(),
                picture: p.clone(),
                bytes: vec![],
                order: position,
            },
        );
    }
    pictures.set(pict);
}

fn load_files(files: Option<web_sys::FileList>) -> Vec<File> {
    let mut result = vec![];
    if let Some(list) = files {
        for i in 0..list.length() {
            if let Some(file) = list.get(i) {
                let gf = gloo::file::File::from(file);
                if gf.raw_mime_type().eq("image/jpeg") {
                    result.push(gf);
                }
            }
        }
    }
    result
}
