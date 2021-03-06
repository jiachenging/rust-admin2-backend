#![recursion_limit="4096"]

#[macro_use] extern crate lucky;
//#[macro_use] extern crate serde_json;

use wasm_bindgen::prelude::*;
use yew::prelude::*;

//#[global_allocator]
//static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub mod pages;
pub mod layouts;

#[wasm_bindgen(start)]
pub async fn run_app() {
    //web_logger::init();
    App::<layouts::LayoutDefaults>::new().mount_to_body();
}
