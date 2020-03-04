use appbar;
use buttons;
use caret;
use checkbox;
use container;
use divider;
use form;
use grid;
use panel;
use yew::prelude::*;

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        App
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <appbar::App/>
                <buttons::App/>
                <caret::App/>
                <checkbox::App/>
                <container::App/>
                <divider::App/>
                <form::App/>
                <grid::App/>
                <panel::App/>
            </div>
        }
    }
}
