use yew::prelude::*;
use yew::services::fetch::{FetchTask};
use yew::virtual_dom::{VNode, VList};
use lucky::{
    json::JsonPager,
    crypt,
    models::{UserRow},
    types::SwapData,
    web::{FetchMsg, Console, Api, LocalStorage},
};
use js_sys::Date;

pub struct Users {
    fetching: bool,
    #[allow(dead_code)]
    link: ComponentLink<Self>,
    #[allow(dead_code)]
    task: Option<FetchTask>,
    pager: Option<JsonPager<UserRow>>,
}

impl Component for Users {

    type Message = FetchMsg<SwapData>;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let pager = if let Some(v) = LocalStorage::get::<JsonPager<UserRow>>("users") {
            if Date::now() / 1000.0 - v.time < 30.0 { //如果缓存在30秒之内
                Some(v)
            } else { None }
        } else { None };
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
                let data = if let Ok(v) = response { v } else {
                    Console::error("获取服务端返回的数据时出错");
                    self.fetching = false;
                    return false;
                };
                let pager = if let Ok(mut v) = crypt::decrypt::<JsonPager<UserRow>>(&data) {
                    v.time = Date::now() / 1000.0;
                    LocalStorage::set::<JsonPager<UserRow>>("users", &v);
                    Some(v)
                } else {
                    Console::error("对于返回信息进行解密失败");
                    self.fetching = false;
                    return false;
                };
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
        let mut rows = VList::new();
        if let Some(p) = &self.pager {
            if let Some(rs) = &p.rows {
                for r in rs {
                    rows.add_child(html!{
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
                    });
                }
            }
        }

        render_view!("/users/users.html")
    }
}
