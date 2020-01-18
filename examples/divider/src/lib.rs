use muicss_yew::components::divider::Divider;
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
                <h1> { "Divider" } </h1>
                <div class="mui-panel">
                    <div id="example">
                        <Divider />
                    </div>
                </div>
            </div>
        }
    }
}
