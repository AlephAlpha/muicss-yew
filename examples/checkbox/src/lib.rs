use muicss_yew::checkbox::Checkbox;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

pub struct CheckboxExample;

impl Component for CheckboxExample {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        CheckboxExample
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
                <div id="example1">
                    <Checkbox label="Option one" value="opt1" />
                </div>
                <div id="example2">
                    <Checkbox label="Option two" value="opt2" checked=true />
                </div>
                <div id="example3">
                    <Checkbox label="Option three"
                        value="opt3"
                        checked=true
                        disabled=true />
                </div>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<CheckboxExample>::new().mount_to_body();
}
