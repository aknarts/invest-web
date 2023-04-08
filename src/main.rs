mod app;
mod components;
mod pages;

use app::App;
use tracing_wasm::WASMLayerConfigBuilder;
mod error;
mod hooks;
mod services;
mod types;
mod utils;

fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default_with_config(
        WASMLayerConfigBuilder::new()
            .set_max_level(tracing::Level::DEBUG)
            .build(),
    );
    yew::Renderer::<App>::new().render();
}
