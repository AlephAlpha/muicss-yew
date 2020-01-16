use muicss_yew::components::container::Container;
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
            <>
                <div id="container-example1">
                    <Container>
                        <h1> { "Container" } </h1>
                        <div className="mui-panel">
                        </div>
                    </Container>
                </div>
                <div id="container-example2">
                    <Container fluid=true>
                        <h1> { "Fluid Container" } </h1>
                        <div className="mui-panel"></div>
                    </Container>
                </div>
            </>
        }
    }
}
