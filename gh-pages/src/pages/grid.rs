use crate::components::example::Example;
use muicss_yew::{col::Col, container::Container, row::Row};
use yew::prelude::*;
use yew_prism::Prism;

#[derive(Clone, Debug)]
pub struct GridExamples;

impl Component for GridExamples {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        GridExamples
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                { self.introduction() }
                { self.example1() }
                { self.example2() }
                { self.example3() }
            </>
        }
    }
}

impl GridExamples {
    const IMPORT: &'static str = r#"use muicss_yew::container::Container;
use muicss_yew::col::Col;
use muicss_yew::row::Row;"#;

    fn introduction(&self) -> Html {
        html! {
            <>
                <h1>
                    { "Grid" }
                </h1>
                <Prism code=Self::IMPORT language="rust" />
                <p>
                    { "See " }
                    <a href="https://www.muicss.com/docs/v1/css-js/grid">
                        { "MUI's website" }
                    </a>
                    { " for details." }
                </p>
            </>
        }
    }

    fn example1(&self) -> Html {
        html! {
            <>
                <h2>
                    { "Example: Stacked-to-horizontal" }
                </h2>
                <Example code=include_str!("../examples/grid_example1.rs")>
                    { include!("../examples/grid_example1.rs") }
                </Example>
            </>
        }
    }

    fn example2(&self) -> Html {
        html! {
            <>
                <h2>
                    { "Example: Mobile and desktop" }
                </h2>
                <Example code=include_str!("../examples/grid_example2.rs")>
                    { include!("../examples/grid_example2.rs") }
                </Example>
            </>
        }
    }

    fn example3(&self) -> Html {
        html! {
            <>
                <h2>
                    { "Example: Offsetting columns" }
                </h2>
                <Example code=include_str!("../examples/grid_example3.rs")>
                    { include!("../examples/grid_example3.rs") }
                </Example>
            </>
        }
    }
}
