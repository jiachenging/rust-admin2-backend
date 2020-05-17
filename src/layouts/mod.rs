use serde_derive::{Deserialize, Serialize};
use yew::format::{Json}; //, Nothing};
use yew::services::fetch::{FetchService, FetchTask};
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew::services::ConsoleService;
use crate::common::{FetchMsg, crypt};

const REGISTER_URL: &'static str = "http://yobet.local/member/index/register";

#[derive(Serialize, Deserialize, Debug)]
pub struct DataFromFile {
    errcode: u32,
    message: String,
    data: Option<String>,
}

/// 默认布局
pub struct LayoutDefaults {
    link: ComponentLink<Self>,
    fetch_service: FetchService,
    fetch_task: Option<FetchTask>,
    fetching: bool,
    data: Option<DataFromFile>,
    console: ConsoleService,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserLogin { 
    user_name: String,
    password: String,
}

impl Component for LayoutDefaults {

    type Message = FetchMsg<DataFromFile>;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            fetch_service: FetchService::new(),
            fetch_task: None,
            fetching: false,
            data: None,
            console: ConsoleService::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            FetchMsg::FetchData => {
                self.fetching = true; // 4.
                let callback = self.link.callback(fetch_callback!());
                let login_info = UserLogin { 
                    user_name: "tempname".to_owned(),
                    password: "qwe123".to_owned(),
                };
                let json_data = match crypt::encrypt(&login_info)  {
                    Ok(v) => { v },
                    Err(err) => {
                        self.console.log(&format!("err: {}", err));
                        return false;
                    }
                };
                let des_string = match crypt::decrypt_string(&json_data.data) {
                    Ok(v) => { v },
                    Err(err) => {
                        self.console.log(err);
                        return false;
                    }
                };
                self.console.log(&format!("{:#?}", des_string));
                let result = crypt::decrypt::<UserLogin>(&des_string);
                self.console.log(&format!("{:#?}", result));
                let json = if let Ok(v) = serde_json::to_string(&json_data) { v } else { self.console.log("=二"); return false; };
                self.console.log("====三");
                let request = yew::services::fetch::Request::post(REGISTER_URL).body(Json(&json)).unwrap();
                //let request = fetch_post!(REGISTER_URL);
                let task = self.fetch_service.fetch(request, callback).unwrap();
                self.fetch_task = Some(task);
            }
            FetchMsg::FetchReady(response) => {
                self.fetching = false; // 4.
                self.data = response.ok(); // 6.
                let result = format!("result: {:?}", &self.data);
                self.console.log(result.as_str());
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
