use muicss_yew::components::caret::{Caret, Direction};
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
            <div class="mui-container">
                <h1> { "Caret" } </h1>
                <div class="mui-panel">
                    <div id="caret-down">
                        <Caret />
                    </div>
                    <div id="caret-up">
                        <Caret direction=Some(Direction::Up) />
                    </div>
                    <div id="caret-right">
                        <Caret direction=Some(Direction::Right) />
                    </div>
                    <div id="caret-left">
                        <Caret direction=Some(Direction::Left) />
                    </div>
                </div>
            </div>
        }
    }
}
