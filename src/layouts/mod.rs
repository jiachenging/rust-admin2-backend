use yew::services::fetch::{FetchService, FetchTask};
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use lucky::{
    crypt,
    {web::{FetchMsg, Dialog, Validation, validation::Rules, Query, Console, LocalStorage}, types::SwapData},
    models::index::{LoginInfo, LoginResult}
};
use crate::pages::index::IndexMain;

const REGISTER_URL: &'static str = "/login";

/// 默认布局
pub struct LayoutDefaults {
    link: ComponentLink<Self>,
    fetch_service: FetchService,
    fetch_task: Option<FetchTask>,
    fetching: bool,
    validator: Validation,
    has_logged: bool,
}

impl Component for LayoutDefaults {

    type Message = FetchMsg<SwapData>;
    type Properties = ();

    /// create
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut validator = Validation::new();
        validator.add("username", Rules::UserName, "必须输入用户名称")
            .add("password", Rules::Password, "必须输入用户密码");

        let has_logged = if let Some(v) = LocalStorage::get::<LoginInfo>("login_info") {
            v.username != ""
        } else { false };
        Self {
            link,
            fetch_service: FetchService::new(),
            fetch_task: None,
            fetching: false,
            validator,
            has_logged,
        }
    }

    /// update
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            FetchMsg::FetchData => {
                if self.fetching {
                    Dialog::alert("请不要重复提交");
                    return false;
                }
                self.fetching = true; // 4.
                match self.validator.validate() {
                    Ok(_) => {
                        let callback = self.link.callback(fetch_callback!());
                        let username = Query::value_by_id("username");
                        let password = Query::value_by_id("password");
                        let login_info = LoginInfo { username, password, };
                        let data = crypt::encrypt_to_value(&login_info);
                        //let data = encrypt_struct!(LoginInfo :: &login_info);
                        let request = request_post!(REGISTER_URL, &data);
                        let task = self.fetch_service.fetch(request, callback).unwrap();
                        self.fetch_task = Some(task);
                    },
                    Err(err) => {
                        Dialog::alert(&err);
                        self.fetching = false;
                        return false;
                    }
                };
            }
            FetchMsg::FetchReady(response) => {
                self.fetching = false; // 已经读取成功
                if let Some(v) = response.ok() {
                    let login_result = if let Ok(v) = crypt::decrypt::<LoginResult>(&v) { v } else {
                        Console::log("解密下发的数据失败");
                        return false;
                    };
                    let message = format!("login-result: {:?}", &login_result);
                    Console::log(&message);
                    Console::log_object(&login_result);
                    if login_result.code != 0 {
                        Dialog::alert(&login_result.message);
                        return false;
                    }
                    self.has_logged = true;
                    return true;
                }
            }
            FetchMsg::FetchError => {
                return false;
            }
        }

        true
    }

    /// change
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    /// view
    fn view(&self) -> Html {
        render_layout!("/default.html")
    }
}
