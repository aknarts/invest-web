use super::picture::Picture;
use gloo::file::File;
use tracing::debug;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::{html, use_node_ref, use_state, Callback, Html, UseStateHandle};

#[derive(Clone, Debug, PartialEq)]
pub struct PictureInfo {
    pub name: String,
    pub mime: String,
    pub picture: File,
}

#[function_component(Pictures)]
pub fn pictures() -> Html {
    let pictures: UseStateHandle<Vec<Html>> = use_state(Vec::new);

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

            <button type="button" class="btn btn-outline-secondary btn-lg p-5 mb-3 w-100" onclick={click_add_image}
                    ondrop={on_image_drop}
                    ondragover={Callback::from(|event: DragEvent| {
                        event.prevent_default();
                    })}
                    ondragenter={Callback::from(|event: DragEvent| {
                        event.prevent_default();
                    })}>{"Drag or Click"}</button>
            <input ref={file_picker} type="file" accept="image/jpeg" style="display:none;" onchange={on_image_select} multiple={true}/>
            <div key={pics.len()}>
                { for pics.iter().enumerate().map(|(i, n)| {
                        html!(
                            <ContextProvider<usize> context={i}>
                                { n.clone() }
                            </ContextProvider<usize>>
                        )
                    })
                }
            </div>
        </>
    )
}

fn process_pictures(pictures: &UseStateHandle<Vec<Html>>, input: Option<web_sys::FileList>) {
    let pic = load_files(input);
    let mut pict = (**pictures).clone();
    for p in pic {
        let name = p.name().clone();
        debug!("Processing: {name}");
        let mime = p.raw_mime_type().clone();
        let data = PictureInfo {
            name: name.clone(),
            mime: mime.clone(),
            picture: p.clone(),
        };
        pict.push(html!(<Picture data={data.clone()}></Picture>));
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
