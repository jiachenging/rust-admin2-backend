use yew::prelude::*;
use crate::pages::index::{IndexLogin, IndexMain};

/// 默认布局
pub struct LayoutDefaults {
    logged_in: bool //是否已经登录
}

impl Component for LayoutDefaults {

    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            logged_in: false
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        render_layout!("/default.html")
    }
}