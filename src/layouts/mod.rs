//use serde_derive::{Deserialize, Serialize};
use yew::format::{Json}; //, Nothing};
use yew::services::fetch::{FetchService, FetchTask};
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew::services::ConsoleService;
use army_common::{self, {web::{FetchMsg}, crypt, types::SwapData}, models::index::{LoginInfo, LoginResult}};

const REGISTER_URL: &'static str = "http://admin.army.rs/api/v1/login";

/// 默认布局
pub struct LayoutDefaults {
    link: ComponentLink<Self>,
    fetch_service: FetchService,
    fetch_task: Option<FetchTask>,
    fetching: bool,
    data: Option<SwapData>,
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
            data: None,
            console: ConsoleService::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            FetchMsg::FetchData => {
                self.fetching = true; // 4.
                let callback = self.link.callback(fetch_callback!());
                let login_info = LoginInfo {
                    username: "temp_name".to_owned(),
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
                let result = crypt::decrypt::<LoginInfo>(&des_string);
                self.console.log(&format!("{:#?}", result));
                let json = if let Ok(v) = serde_json::to_string(&json_data) { v } else { self.console.log("=二"); return false; };
                let json_str = format!("json: {}", &json);
                self.console.log(json_str.as_str());
                //let ss: &[u8] = json.as_bytes();
                let body_json = Json(&json);
                let body_str = format!("body: {:?}", &body_json);
                self.console.log(body_str.as_str());
                let json_2 = json!({"data": json_data.data});
                //let json_2 = serde_json::Value::String(json);
                let request = yew::services::fetch::Request::post(REGISTER_URL)
                    .header("content-type", "application/json;charset=UTF-8")
                    .body(Json(&json_2))
                    //.body(&json)
                    .unwrap();
                //let (parts, body) = request.into_parts();
                //let body = serde_json::to_vec(&body).unwrap();
                //let request = yew::services::fetch::Request::from_parts(parts, body);
                let task = self.fetch_service.fetch(request, callback).unwrap();
                self.fetch_task = Some(task);
            }
            FetchMsg::FetchReady(response) => {
                self.fetching = false; // 4.
                self.data = response.ok(); // 6.
                let result = format!("result: {:?}", &self.data);
                self.console.log(result.as_str());
                if let Some(v) = &self.data { 
                    let data_str = &v.data;   
                    let decrypt_str = crypt::decrypt_string(&data_str).unwrap();
                    let login_result: LoginResult = crypt::decrypt(&decrypt_str).unwrap();
                    let result_str = format!("login-result: {:?}", login_result);
                    self.console.log(result_str.as_str());
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
