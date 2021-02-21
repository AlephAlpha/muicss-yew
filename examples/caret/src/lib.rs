use muicss_yew::caret::{Caret, Direction};
use wasm_bindgen::prelude::*;
use yew::prelude::*;

pub struct AppbarExample;

impl Component for AppbarExample {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        AppbarExample
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <div>
                    <Caret />
                </div>
                <div>
                    <Caret direction=Direction::Up />
                </div>
                <div>
                    <Caret direction=Direction::Right />
                </div>
                <div>
                    <Caret direction=Direction::Left />
                </div>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<AppbarExample>::new().mount_to_body();
}
