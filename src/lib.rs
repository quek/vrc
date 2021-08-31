use wasm_bindgen::prelude::*;

pub mod app;
pub mod fetcher;

#[wasm_bindgen]
pub fn run() {
    yew::start_app::<app::Model>();
}
