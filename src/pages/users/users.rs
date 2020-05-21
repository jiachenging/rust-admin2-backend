use yew::prelude::*;
use yew::services::fetch::{FetchTask};
use yew::virtual_dom::{VNode};
use lucky::{
    json::JsonPager,
    models::{UserRow},
    types::SwapData,
    web::{FetchMsg, Console, Api},
    html::DataTable,
};

pub struct Users {
    fetching: bool,
    #[allow(dead_code)]
    link: ComponentLink<Self>,
    #[allow(dead_code)]
    task: Option<FetchTask>,
    pager: Option<JsonPager<UserRow>>,
}

impl DataTable for Users {}

impl Component for Users {

    type Message = FetchMsg<SwapData>;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let pager = Self::get_pager::<UserRow>("users");
        Self {
            task: if pager.is_none() { Some(Api::send("/users", &link)) } else { None },
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
                let pager = Self::fetch_pager::<UserRow>(response, "users");
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
        let rows = Self::init_rows::<UserRow>(&self.pager, |r: &UserRow| {
            return html!{
                <tr>
                    <td>{r.id}</td>
                    <td>{&r.name}</td>
                    <td>{&r.login_count}</td>
                    <td>{&r.last_login}</td>
                    <td>{&r.last_ip}</td>
                    <td>{if r.status == 1 { "正常"} else { "停用"}}</td>
                    <td>{&r.created}</td>
                    <td>{&r.updated}</td>
                    <td>{"修改 | 删除"}</td>
                </tr>
            };
        });
        render_view!("/users/users.html")
    }
}


