use muicss_yew::appbar::Appbar;
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
            <Appbar></Appbar>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<AppbarExample>::new().mount_to_body();
}
