use super::picture::Picture;
use gloo::file::File;
use tracing::{debug, warn};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::{html, use_node_ref, use_state, Callback, Html, UseStateHandle};
use yew_hooks::use_counter;

#[derive(Clone, Debug, PartialEq)]
pub struct PictureInfo {
    pub name: String,
    pub mime: String,
    pub picture: File,
}

#[function_component(Pictures)]
pub fn pictures() -> Html {
    let pictures: UseStateHandle<Vec<Html>> = use_state(Vec::new);
    let drag_over = use_counter(0);

    let on_image_select = {
        let pictures = pictures.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            process_pictures(&pictures, input.files());
        })
    };

    let on_image_drop = {
        let pictures = pictures.clone();
        let drag_over = drag_over.clone();
        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            drag_over.set(0);
            if let Some(input) = e.data_transfer() {
                process_pictures(&pictures, input.files());
                if let Err(e) = input.clear_data() {
                    warn!("Unable to clear drag data: {:?}", e);
                };
            };
        })
    };

    let on_drag_enter = {
        let drag_over = drag_over.clone();
        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            if let Some(input) = e.data_transfer() {
                if validate_list(input.items()).is_none() {
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
                if validate_list(input.items()).is_none() {
                    return;
                }
                drag_over.decrease();
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
    let drag_over_class = if (*drag_over) > 0 {
        Some("btn-secondary")
    } else {
        Some("btn-outline-secondary")
    };
    html!(<>
            <div class="h5">
                {"Pictures"}
            </div>

            <button type="button" class={classes!("btn", drag_over_class, "btn-lg", "p-5", "mb-3", "w-100")} onclick={click_add_image}
                    ondrop={on_image_drop}
                    ondragover={|e: DragEvent| e.prevent_default() }
                    ondragleave={on_drag_leave}
                    ondragenter={on_drag_enter}>{"Drag or Click"}</button>
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

fn validate_list(list: web_sys::DataTransferItemList) -> Option<i32> {
    if list.length() < 1 {
        debug!("Too little files");
        return None;
    }
    let mut num = 0;
    for i in 0..list.length() {
        if let Some(item) = list.get(i) {
            if item.kind().eq("file") {
                num += 1;
            }
        }
    }
    if num > 0 {
        Some(num)
    } else {
        None
    }
}
