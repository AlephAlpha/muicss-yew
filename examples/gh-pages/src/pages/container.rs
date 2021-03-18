use crate::components::example::Example;
use muicss_yew::container::Container;
use yew::prelude::*;
use yew_prism::Prism;

#[derive(Clone, Debug)]
pub struct ContainerExamples;

impl Component for ContainerExamples {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        ContainerExamples
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
            </>
        }
    }
}

impl ContainerExamples {
    const IMPORT: &'static str = r#"use muicss_yew::container::Container;"#;

    fn introduction(&self) -> Html {
        html! {
            <>
                <h1>
                    { "Containers" }
                </h1>
                <Prism code=Self::IMPORT language="rust" />
                <p>
                    { "See " }
                    <a href="https://www.muicss.com/docs/v1/css-js/container">
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
                    { "Fixed Container" }
                </h2>
                <Example code=include_str!("../examples/container_example1.rs")>
                    { include!("../examples/container_example1.rs") }
                </Example>
            </>
        }
    }

    fn example2(&self) -> Html {
        html! {
            <>
                <h2>
                    { "Fluid Container" }
                </h2>
                <Example code=include_str!("../examples/container_example2.rs")>
                    { include!("../examples/container_example2.rs") }
                </Example>
            </>
        }
    }
}
