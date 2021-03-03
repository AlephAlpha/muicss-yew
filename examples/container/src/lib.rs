use muicss_yew::container::Container;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

pub struct ContainerExample;

impl Component for ContainerExample {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        ContainerExample
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
                <Container>
                    { "Container" }
                </Container>
                <Container  fluid=true>
                    { "Fluid Container" }
                </Container>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<ContainerExample>::new().mount_to_body();
}
