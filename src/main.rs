#![recursion_limit="2048"]

//#[global_allocator]
//static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use yew::prelude::*;

#[macro_use]
mod common;
mod pages;
mod layouts;
//mod router;

use layouts::LayoutDefaults;

/// 主函数
fn main() {
    yew::initialize();
    App::<LayoutDefaults>::new().mount_to_body();
}
