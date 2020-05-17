#![recursion_limit="2048"]
//#[macro_use] extern crate log;
//extern crate web_logger;

use wasm_bindgen::prelude::*;
//#[global_allocator]
//static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
use yew::prelude::*;

#[macro_use]
pub mod common;
pub mod pages;
pub mod layouts;
//mod router;

use layouts::LayoutDefaults;

#[wasm_bindgen(start)]
pub async fn run_app() {
    // web_logger::init();
    App::<LayoutDefaults>::new().mount_to_body();
}
