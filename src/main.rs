mod app;
mod components;
mod pages;
use app::App;
mod error;
mod hooks;
mod services;
mod types;

fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();
    yew::Renderer::<App>::new().render();
}
