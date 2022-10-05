mod app;
mod components;
mod pages;
use app::App;
mod error;
mod hooks;
mod services;
mod types;

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::Renderer::<App>::new().render();
}
