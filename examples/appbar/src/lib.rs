use muicss_yew::components::appbar::Appbar;
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
            <Appbar class=Classes::from("test")>
                <div class="mui-container">
                    <h1> { "Appbar" } </h1>
                </div>
            </Appbar>
        }
    }
}
