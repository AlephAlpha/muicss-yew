#![recursion_limit = "256"]

mod components;
mod pages;

use crate::{
    components::{header::Header, main::Main},
    pages::home::Home,
};
use yew::prelude::*;

pub struct Model;

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Model
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <Header/>
                <Main>
                    <Home/>
                </Main>
            </>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
