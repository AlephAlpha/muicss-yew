use crate::{
    components::{header::Header, main::Main},
    pages::{appbar::AppbarExamples, home::Home},
};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Debug, Switch)]
pub enum AppRoute {
    #[to = "/appbar"]
    Appbar,
    #[to = "/"]
    Home,
}

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
                <Header />
                <Main>
                    <Router<AppRoute> render=Router::render(Self::switch) />
                </Main>
            </>
        }
    }
}

impl Model {
    fn switch(switch: AppRoute) -> Html {
        match switch {
            AppRoute::Appbar => html! { <AppbarExamples /> },
            AppRoute::Home => html! { <Home /> },
        }
    }
}
