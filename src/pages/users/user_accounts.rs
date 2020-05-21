use yew::prelude::*;
use yew::services::fetch::{FetchTask};
use yew::virtual_dom::{VNode};
use lucky::{
    json::JsonPager,
    models::{UserAccountRow},
    types::SwapData,
    web::{FetchMsg, Api},
    html::DataTable,
};

const API_LIST: &'static str = "/user_accounts";
const API_CACHE_KEY: &'static str = "user_accounts";

pub struct UserAccounts {
    fetching: bool,
    #[allow(dead_code)]
    link: ComponentLink<Self>,
    #[allow(dead_code)]
    task: Option<FetchTask>,
    pager: Option<JsonPager<UserAccountRow>>,
}

impl DataTable for UserAccounts {}

impl Component for UserAccounts {

    type Message = FetchMsg<SwapData>;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let pager = Self::get_pager::<UserAccountRow>(API_CACHE_KEY);
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
                let pager = Self::fetch_pager::<UserAccountRow>(response, API_CACHE_KEY);
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
        let rows = Self::init_rows(&self.pager, |r: &UserAccountRow| {
            return html!{
                <tr>
                    <td>{&r.id}</td>
                    <td>{&r.user_id}</td>
                    <td>{&r.user_name}</td>
                    <td>{format!("{:.4}", &r.frozen)}</td>
                    <td>{format!("{:.4}", &r.remain)}</td>
                    <td>{format!("{:.4}", &r.total)}</td>
                    <td>{format!("{:.4}", &r.total_in)}</td>
                    <td>{format!("{:.4}", &r.total_out)}</td>
                    <td>{""}</td>
                </tr>
            };
        });
        render_view!("/users/user_accounts.html")
    }
}
