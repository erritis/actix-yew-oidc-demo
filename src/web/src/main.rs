mod app;
mod route;
mod pages;
mod components;
mod services;
mod opts;

use app::App;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
