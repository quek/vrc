use wasm_bindgen::prelude::*;

pub mod app;

#[wasm_bindgen]
pub fn run() {
    yew::start_app::<app::Model>();
}
