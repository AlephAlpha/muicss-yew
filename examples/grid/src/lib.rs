#![recursion_limit = "512"]

use muicss_yew::components::{col::Col, row::Row};
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
                <h1> { "Grid" } </h1>
                <div class="mui-panel">
                    <div id="example1">
                        { example1() }
                    </div>
                    <hr />
                    <div id="example2">
                        { example2() }
                    </div>
                </div>
            </div>
        }
    }
}

fn example1() -> Html {
    html! {
        <div>
            <Row>
                <Col md=Some(1)> { "md-1" } </Col>
                <Col md=Some(1)> { "md-1" } </Col>
                <Col md=Some(1)> { "md-1" } </Col>
                <Col md=Some(1)> { "md-1" } </Col>
                <Col md=Some(1)> { "md-1" } </Col>
                <Col md=Some(1)> { "md-1" } </Col>
                <Col md=Some(1)> { "md-1" } </Col>
                <Col md=Some(1)> { "md-1" } </Col>
                <Col md=Some(1)> { "md-1" } </Col>
                <Col md=Some(1)> { "md-1" } </Col>
                <Col md=Some(1)> { "md-1" } </Col>
                <Col md=Some(1)> { "md-1" } </Col>
            </Row>
            <Row>
                <Col md=Some(8)> { "md-8" } </Col>
                <Col md=Some(4)> { "md-4" } </Col>
            </Row>
            <Row>
                <Col md=Some(4)> { "md-4" } </Col>
                <Col md=Some(4)> { "md-4" } </Col>
                <Col md=Some(4)> { "md-4" } </Col>
            </Row>
            <Row>
                <Col md=Some(6)> { "md-6" } </Col>
                <Col md=Some(6)> { "md-6" } </Col>
            </Row>
        </div>
    }
}

fn example2() -> Html {
    html! {
        <div>
            <Row>
                <Col md=Some(4)>
                    { "md-4" }
                </Col>
                <Col md=Some(4) md_offset=Some(4)>
                    { "md-4 md-offset-4" }
                </Col>
            </Row>
            <Row>
                <Col md=Some(3) md_offset=Some(3)>
                    { "md-3 md-offset-3" }
                </Col>
                <Col md=Some(3) md_offset=Some(3)>
                    { "md-3 md-offset-3" }
                </Col>
            </Row>
            <Row>
                <Col md=Some(6) md_offset=Some(3)>
                    { "md-6 md-offset-3" }
                </Col>
            </Row>
        </div>
    }
}
