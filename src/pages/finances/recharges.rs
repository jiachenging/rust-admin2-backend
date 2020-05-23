use yew::prelude::*;
use yew::services::fetch::{FetchTask};
use yew::virtual_dom::{VNode};
use lucky::{
    json::JsonPager,
    models::{RechargeRow as ModelRow},
    types::SwapData,
    web::{FetchMsg, Api},
    html::DataTable,
};

const API_LIST: &'static str = "/recharges";
const API_CACHE_KEY: &'static str = "recharges";

pub struct Recharges {
    fetching: bool,
    #[allow(dead_code)]
    link: ComponentLink<Self>,
    #[allow(dead_code)]
    task: Option<FetchTask>,
    pager: Option<JsonPager<ModelRow>>,
}

impl DataTable for Recharges {}

impl Component for Recharges {

    type Message = FetchMsg<SwapData>;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let pager = Self::get_pager::<ModelRow>(API_CACHE_KEY);
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
                let pager = Self::fetch_pager::<ModelRow>(response, API_CACHE_KEY);
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
        let rows = Self::init_rows(&self.pager, |r: &ModelRow| {
            return html!{
                <tr>
                    <td>{&r.id}</td>
                    <td>{&r.bill_no}</td>
                    <td>{&r.user_id}</td>
                    <td>{&r.user_name}</td>
                    <td>{&r.amount}</td>
                    <td>{&r.status}</td>
                    <td>{&r.audit_admin_id}</td>
                    <td>{&r.audit_admin_name}</td>
                    <td>{&r.created}</td>
                    <td>{""}</td>
                </tr>
            };
        });
        render_view!("/finances/recharges.html")
    }
}

