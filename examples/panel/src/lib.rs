use muicss_yew::components::panel::Panel;
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
                <h1> { "Panel" } </h1>
                <div id="example">
                    <Panel />
                </div>
            </div>
        }
    }
}
