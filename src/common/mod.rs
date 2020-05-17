use serde_derive::{Serialize, Deserialize};

/// 获取数据处理状态
pub enum FetchMsg<T: serde::ser::Serialize> {
    FetchData,
    FetchReady(Result<T, anyhow::Error>), // 2.
    FetchError,
}

/// 全站统一数据处理
#[derive(Serialize, Deserialize)]
pub struct JsonData { 
    pub data: String,
}

/// 调用布局文件
#[macro_export]
macro_rules! render_layout {
    ($path: expr) => ({
        include!(concat!("../templates/layouts", $path))
    })
}

/// 调用页面文件
#[macro_export]
macro_rules! render_view {
    ($path: expr) => ({
        include!(concat!("../../templates/pages", $path))
    })
}

#[macro_export]
macro_rules! fetch_callback {
    () => {
        move |response: yew::services::fetch::Response<yew::format::Json<Result<DataFromFile, anyhow::Error>>>| { // 2.
            let (meta, Json(data)) = response.into_parts();
            return if meta.status.is_success() { crate::common::FetchMsg::FetchReady(data) } else { crate::common::FetchMsg::FetchError };
        }
    }
}

//#[macro_export]
//macro_rules! fetch_post {
//    ($url: expr) => ({
//        yew::services::fetch::Request::post($url).body("{}").unwrap()
//        //request.post($url).unwrap()
//        //body.push_str("{\"code\": 1}");
//        //builder
//    })
//}

pub mod crypt;
