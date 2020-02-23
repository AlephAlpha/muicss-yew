use muicss_yew::components::checkbox::Checkbox;
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
                <h1> { "Checkbox" } </h1>
                <div class="mui-panel">
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
            </div>
        }
    }
}
