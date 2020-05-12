use yew::prelude::*;
use super::{IndexHeader, IndexMenus};
use crate::pages::users::index::UsersIndex;

pub struct IndexMain { }

impl Component for IndexMain {

    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        render_view!("/index/main.html")
    }
}