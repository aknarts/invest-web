use std::rc::Rc;
use super::picture::Picture;
use gloo::file::File;
use tracing::{debug, warn};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::{html, use_node_ref, use_state, Callback, Html, UseStateHandle};
use yew_hooks::use_counter;
use crate::pages::admin::investments::modal::InvestmentInfo;

#[derive(Clone, Debug, PartialEq)]
pub struct PictureInfo {
    pub name: String,
    pub mime: String,
    pub picture: File,
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct PicturesStruct {
    nodes: Vec<Html>,
    counter: i32,
}

pub enum PicturesActions {
    Add(Html),
    Move(usize, usize),
}

impl Reducible for PicturesStruct {
    type Action = PicturesActions;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut new = (*self).nodes.clone();
        match action {
            PicturesActions::Add(node) => {
                new.push(node);
            }
            PicturesActions::Move(from, to) => {
                let temp = new.remove(from);
                new.insert(to, temp);
                debug!("Moving {from} to {to}");
            }
        };
        Self { nodes: new, counter: self.counter + 1 }.into()
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub dispatcher: UseReducerDispatcher<InvestmentInfo>,
}

#[function_component(Pictures)]
pub fn pictures(props: &Props) -> Html {
    let dispatcher = props.dispatcher.clone();
    let updates = use_counter(0);
    let pictures = use_reducer(PicturesStruct::default);
    let drag_over = use_counter(0);

    let on_image_select = {
        let pictures = pictures.dispatcher();
        let updates = updates.clone();
        let uploads = dispatcher.clone();

        use_callback(
            move |e: Event, uploads| {
                let input: HtmlInputElement = e.target_unchecked_into();
                process_pictures(&pictures, uploads, input.files());
                updates.increase();
            },
            uploads,
        )
    };

    let on_image_drop = {
        let pictures = pictures.dispatcher();
        let updates = updates.clone();
        let drag_over = drag_over.clone();
        let uploads = dispatcher;
        use_callback(
            move |e: DragEvent, uploads| {
                e.prevent_default();
                drag_over.set(0);
                if let Some(input) = e.data_transfer() {
                    process_pictures(&pictures, uploads, input.files());
                    if let Err(e) = input.clear_data() {
                        warn!("Unable to clear drag data: {:?}", e);
                    };
                    updates.increase();
                };
            },
            uploads,
        )
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

    let pics = (*pictures).nodes.clone();
    let drag_over_class = if (*drag_over) > 0 {
        Some("btn-secondary")
    } else {
        Some("btn-outline-secondary")
    };

    let key = *updates + (*pictures).counter;
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
            <div key={key}>
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

fn process_pictures(
    pictures: &UseReducerDispatcher<PicturesStruct>,
    uploads: &UseReducerDispatcher<InvestmentInfo>,
    input: Option<web_sys::FileList>,
) {
    let pic = load_files(input);
    for p in pic {
        let name = p.name().clone();
        debug!("Processing: {name}");
        let mime = p.raw_mime_type().clone();
        let data = PictureInfo {
            name: name.clone(),
            mime: mime.clone(),
            picture: p.clone(),
        };
        pictures.dispatch(PicturesActions::Add(html!(<Picture pictures_dispatcher={pictures.clone()} uploads_dispatcher={uploads.clone()} data={data.clone()}></Picture>)));
    }
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
