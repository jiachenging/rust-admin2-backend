use yew::prelude::*;
use yew::services::fetch::{FetchTask};
use lucky::{
    json::JsonPager,
    crypt,
    models::{UserRow},
    types::SwapData,
    web::{FetchMsg, Console, Api},
};

pub struct Users {
    fetching: bool,
    #[allow(dead_code)]
    link: ComponentLink<Self>,
    #[allow(dead_code)]
    task: Option<FetchTask>,
}

impl Users {
    pub fn test() {
        Console::log("测试 test!");
    }
}

impl Component for Users {

    type Message = FetchMsg<SwapData>;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self::test();
        Self {
            task: Some(Api::send("/users", &link)),
            fetching: true,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            FetchMsg::FetchData => {
                if self.fetching { //如果正在获取数据
                    return false;
                }
            },
            FetchMsg::FetchReady(response) => {
                if let Some(r) = response.ok() {
                    let result = if let Ok(v) = crypt::decrypt::<JsonPager<UserRow>>(&r) { v } else {
                        Console::error("对于返回信息进行解密失败");
                        return false;
                    };
                    let message = format!("pager: {:?}", result);
                    Console::log(&message);
                    Console::log_object(&result);
                    if let Some(rows) = result.rows {
                        bind_data!(&rows, [id, name, login_count, last_login, last_ip, status, created, updated]);
                    }
                    self.fetching = false;
                }
            },
            _ => {
                return false;
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        render_view!("/users/users.html")
    }
}
