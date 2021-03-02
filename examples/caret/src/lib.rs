use muicss_yew::caret::{Caret, Direction};
use wasm_bindgen::prelude::*;
use yew::prelude::*;

pub struct CaretExample;

impl Component for CaretExample {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        CaretExample
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
    App::<CaretExample>::new().mount_to_body();
}
