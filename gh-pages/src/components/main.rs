use muicss_yew::{col::Col, container::Container, row::Row};
use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
}

#[derive(Clone, Debug)]
pub struct Main {
    props: Props,
}

impl Component for Main {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Main { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        html! {
            <main>
                <Container fluid=true>
                    <Row>
                        <Col sm=10 sm_offset=1>
                            { self.props.children.clone() }
                        </Col>
                    </Row>
                </Container>
            </main>
        }
    }
}
