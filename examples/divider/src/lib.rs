use muicss_yew::divider::Divider;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

pub struct DividerExample;

impl Component for DividerExample {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        DividerExample
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
                <Divider />
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<DividerExample>::new().mount_to_body();
}
