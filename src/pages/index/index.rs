use yew::prelude::*;
//use super::{IndexHeader};
use crate::pages::{
    users::{ Users, UserAccounts, UserDetails, UserCards, },
    admins::{ AdminMenus, AdminRoles, Admins, AdminLogs },
    finances::{ Recharges, Withdraws, AccountChanges, },
    systems::{ Banks, Provinces, Cities, },
};
use super::IndexRight;


/// 左侧路由菜单
pub enum RoutePages {
    IndexRight,     // 后台首页
    IndexConfig,    // 风站配置
    Users,          // 用户管理
    UserAccounts,   // 用户账户
    UserDetails,    // 用户资料
    UserCards,      // 用户绑卡
    Recharges,      // 充值记录
    Withdraws,      // 提现记录
    AccountChanges, // 账变记录
    Admins,         // 后台用户
    AdminRoles,     // 后台角色
    AdminMenus,     // 后台菜单
    AdminLogs,      // 后台用户日志
    Banks,          // 银行列表
    Provinces,      // 省份列表
    Cities,         // 城市列表
    //Districts,      // 县区列表
}

pub struct IndexMain { 
    route_page: RoutePages,     // 左侧菜单页面
    link: ComponentLink<Self>,  // link
}

impl Component for IndexMain {

    type Message = RoutePages;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { 
            route_page: RoutePages::IndexRight,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        self.route_page = msg;
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        render_view!("/index/main.html")
    }
}
