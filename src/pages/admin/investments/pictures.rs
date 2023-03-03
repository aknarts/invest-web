use super::picture::Picture;
use crate::pages::admin::investments::modal::InvestmentInfo;
use gloo::file::callbacks::FileReader;
use gloo::file::File;
use std::rc::Rc;
use tracing::{debug, error, warn};
use uuid::Uuid;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::{html, use_mut_ref, use_node_ref, Callback, Html};
use yew_hooks::use_counter;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct PictureData {
    pub id: Uuid,
    pub name: String,
    pub mime: String,
    pub path: Option<String>,
    pub bytes: Option<Vec<u8>>,
    pub started_upload: bool,
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct PicturesStruct {
    pictures: Vec<PictureData>,
    counter: i32,
}

pub enum PicturesActions {
    Add(Uuid, String, String),
    UploadStarted(Uuid),
    Uploaded(Uuid, String),
    Loaded(Uuid, Vec<u8>),
    Move(usize, usize),
}

impl Reducible for PicturesStruct {
    type Action = PicturesActions;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut pictures = self.pictures.clone();
        let mut counter = self.counter;
        match action {
            PicturesActions::Add(id, name, mime) => {
                pictures.push(PictureData {
                    id,
                    name,
                    mime,
                    path: None,
                    bytes: None,
                    started_upload: false,
                });
                counter += 1;
            }
            PicturesActions::Move(from, to) => {
                let temp = pictures.remove(from);
                pictures.insert(to, temp);
                counter += 1;
            }
            PicturesActions::Uploaded(id, path) => {
                for picture in &mut pictures {
                    if picture.id.eq(&id) {
                        picture.path = Some(path);
                        break;
                    }
                }
                counter += 1;
            }
            PicturesActions::Loaded(id, data) => {
                for mut picture in &mut pictures {
                    if picture.id.eq(&id) {
                        picture.bytes = Some(data);
                        break;
                    }
                }
                counter += 1;
            }
            PicturesActions::UploadStarted(id) => {
                for mut picture in &mut pictures {
                    if picture.id.eq(&id) {
                        picture.started_upload = true;
                        break;
                    }
                }
            }
        };
        Self { pictures, counter }.into()
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
    let readers = use_mut_ref(Vec::<FileReader>::new);

    let on_image_select = {
        let pictures = pictures.dispatcher();
        let updates = updates.clone();
        let readers = readers.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut loaders = process_pictures(&pictures, input.files());
            while let Some(loader) = loaders.pop() {
                (*readers).borrow_mut().push(loader);
            }
            updates.increase();
        })
    };

    let on_image_drop = {
        let pictures = pictures.dispatcher();
        let updates = updates.clone();
        let drag_over = drag_over.clone();
        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            drag_over.set(0);
            if let Some(input) = e.data_transfer() {
                let mut loaders = process_pictures(&pictures, input.files());
                while let Some(loader) = loaders.pop() {
                    (*readers).borrow_mut().push(loader);
                }
                if let Err(e) = input.clear_data() {
                    warn!("Unable to clear drag data: {:?}", e);
                };
                updates.increase();
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

    let pics = pictures.pictures.clone();
    let drag_over_class = if (*drag_over) > 0 {
        Some("btn-secondary")
    } else {
        Some("btn-outline-secondary")
    };

    let key = *updates + pictures.counter;
    let pictures_dispatcher = pictures.dispatcher();
    let uploads_dispatcher = dispatcher;
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
                                <Picture pictures_dispatcher={pictures_dispatcher.clone()} uploads_dispatcher={uploads_dispatcher.clone()} data={n.clone()}></Picture>
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
    input: Option<web_sys::FileList>,
) -> Vec<FileReader> {
    let pic = load_files(input);
    let mut readers = Vec::new();
    for p in pic {
        let name = p.name().clone();
        debug!("Processing: {name}");
        let mime = p.raw_mime_type().clone();
        let id = Uuid::new_v4();
        pictures.dispatch(PicturesActions::Add(id, name.clone(), mime.clone()));
        readers.push(load_picture(id, p.clone(), pictures));
    }
    readers
}

fn load_picture(
    id: Uuid,
    picture: File,
    pictures: &UseReducerDispatcher<PicturesStruct>,
) -> FileReader {
    let photos_dispatcher = pictures.clone();

    gloo::file::callbacks::read_as_bytes(&picture, move |res| match res {
        Ok(contents) => {
            photos_dispatcher.dispatch(PicturesActions::Loaded(id, contents));
        }
        Err(e) => {
            error!("Unable to read file: {:?}", e);
        }
    })
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
