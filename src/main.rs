#![recursion_limit="2048"]

#[macro_use] extern crate army_common;
//#[global_allocator]
//static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use yew::prelude::*;

mod pages;
mod layouts;
//mod router;

use layouts::LayoutDefaults;

/// 主函数
fn main() {
    yew::initialize();
    App::<LayoutDefaults>::new().mount_to_body();
}
