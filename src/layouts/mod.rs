//use serde_derive::{Deserialize, Serialize};
use yew::format::{Json}; //, Nothing};
use yew::services::fetch::{FetchService, FetchTask};
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew::services::ConsoleService;
use lucky::{self, {web::{FetchMsg}, types::SwapData}, models::index::{LoginInfo, LoginResult}};

const REGISTER_URL: &'static str = "http://admin.army.rs/api/v1/login";

/// 默认布局
pub struct LayoutDefaults {
    link: ComponentLink<Self>,
    fetch_service: FetchService,
    fetch_task: Option<FetchTask>,
    fetching: bool,
    //data: Option<SwapData>,
    console: ConsoleService,
}

impl Component for LayoutDefaults {

    type Message = FetchMsg<SwapData>;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            fetch_service: FetchService::new(),
            fetch_task: None,
            fetching: false,
            //data: None,
            console: ConsoleService::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            FetchMsg::FetchData => {
                self.fetching = true; // 4.
                let callback = self.link.callback(fetch_callback!());
                // 需要检测输入是否正确
                let login_info = LoginInfo {
                    username: "temp_name".to_owned(),
                    password: "qwe123".to_owned(),
                };
                let data = encrypt_struct!(LoginInfo :: &login_info);
                let request = request_post!(REGISTER_URL, &data);
                let task = self.fetch_service.fetch(request, callback).unwrap();
                self.fetch_task = Some(task);
            }
            FetchMsg::FetchReady(response) => {
                self.fetching = false; // 已经读取成功
                if let Some(v) = response.ok() {
                    let login_result = decrypt_struct!(v => LoginResult);
                    let result_str = format!("result: {:?}", login_result);
                    self.console.log(result_str.as_str());
                    let message = format!(r#"
                        layui.use(['layer'], function() {{
                            let layer = layui.layer;
                            layer.confirm('{}');
                        }});
                    "#, login_result.message);
                    let result = js_sys::eval(&message);
                    let msg = format!("result: {:?}", result);
                    self.console.log(&msg);
                    //js!(&message);
                }
            }
            FetchMsg::FetchError => {
                return false;
            }
        }

        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        render_layout!("/default.html")
    }
}
