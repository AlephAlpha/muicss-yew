use crate::{
    components::{header::Header, main::Main},
    pages::{
        appbar::AppbarExamples, buttons::ButtonExamples, caret::CaretExamples,
        checkbox::CheckboxExamples, container::ContainerExamples, divider::DividerExamples,
        dropdowns::DropdownExamples, grid::GridExamples, home::Home, panels::PanelExamples,
    },
    switch::{AppRoute, AppRouter, PublicUrlSwitch},
};
use yew::prelude::*;
use yew_router::prelude::*;

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
                    <AppRouter render=Router::render(Self::switch) />
                </Main>
            </>
        }
    }
}

impl Model {
    fn switch(switch: PublicUrlSwitch) -> Html {
        match switch.route() {
            AppRoute::Appbar => html! { <AppbarExamples /> },
            AppRoute::Buttons => html! { <ButtonExamples /> },
            AppRoute::Caret => html! { <CaretExamples /> },
            AppRoute::Checkbox => html! { <CheckboxExamples /> },
            AppRoute::Container => html! { <ContainerExamples /> },
            AppRoute::Divider => html! { <DividerExamples /> },
            AppRoute::Dropdowns => html! { <DropdownExamples /> },
            AppRoute::Grid => html! { <GridExamples /> },
            AppRoute::Home => html! { <Home /> },
            AppRoute::Panels => html! { <PanelExamples /> },
        }
    }
}
