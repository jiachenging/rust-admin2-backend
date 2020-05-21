use yew::prelude::*;
use yew::services::fetch::{FetchTask};
use yew::virtual_dom::{VNode};
use lucky::{
    json::JsonPager,
    models::{UserDetailRow},
    types::SwapData,
    web::{FetchMsg, Api},
    html::DataTable,
};

const API_LIST: &'static str = "/user_details";
const API_CACHE_KEY: &'static str = "user_details";

pub struct UserDetails {
    fetching: bool,
    #[allow(dead_code)]
    link: ComponentLink<Self>,
    #[allow(dead_code)]
    task: Option<FetchTask>,
    pager: Option<JsonPager<UserDetailRow>>,
}

impl DataTable for UserDetails {}

impl Component for UserDetails {

    type Message = FetchMsg<SwapData>;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let pager = Self::get_pager::<UserDetailRow>(API_CACHE_KEY);
        Self {
            task: if pager.is_none() { Some(Api::send(API_LIST, &link)) } else { None },
            fetching: pager.is_none(), //pager.is_none(), //true,
            link,
            pager,
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
                let pager = Self::fetch_pager::<UserDetailRow>(response, API_CACHE_KEY);
                self.pager = pager;
                self.fetching = false;
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

    fn view(&self) -> VNode {
        let rows = Self::init_rows(&self.pager, |r: &UserDetailRow| {
            return html!{
                <tr>
                    <td>{&r.id}</td>
                    <td>{&r.user_id}</td>
                    <td>{&r.user_name}</td>
                    <td>{&r.gender}</td>
                    <td>{&r.birth}</td>
                    <td>{&r.phone}</td>
                    <td>{&r.mail}</td>
                    <td>{&r.address}</td>
                    <td>{&r.remark}</td>
                    <td>{""}</td>
                </tr>
            };
        });
        render_view!("/users/user_details.html")
    }
}
