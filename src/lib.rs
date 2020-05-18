#![recursion_limit="2048"]
#[macro_use] extern crate army_common;
#[macro_use] extern crate serde_json;

use wasm_bindgen::prelude::*;
use yew::prelude::*;

//#[global_allocator]
//static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub mod pages;
pub mod layouts;

#[wasm_bindgen(start)]
pub async fn run_app() {
    // web_logger::init();
    App::<layouts::LayoutDefaults>::new().mount_to_body();
}
