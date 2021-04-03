use crate::components::example::Example;
use muicss_yew::input::Input;
use yew::prelude::*;
use yew_prism::Prism;

#[derive(Clone, Debug)]
pub struct InputExamples;

impl Component for InputExamples {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        InputExamples
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

impl InputExamples {
    const IMPORT: &'static str = r#"use muicss_yew::input::Input;"#;

    fn introduction(&self) -> Html {
        html! {
            <>
                <h1>
                    { "Input" }
                </h1>
                <Prism code=Self::IMPORT language="rust" />
                <p>
                    { "See " }
                    <a href="https://www.muicss.com/docs/v1/react/input">
                        { "MUI's website" }
                    </a>
                    { " for details. But this Yew component does not support so many \
                       properties as the React component." }
                </p>
            </>
        }
    }

    fn example1(&self) -> Html {
        html! {
            <>
                <h2>
                    { "Basic example" }
                </h2>
                <Example code=include_str!("../examples/input_example1.rs")>
                    { include!("../examples/input_example1.rs") }
                </Example>
            </>
        }
    }

    fn example2(&self) -> Html {
        html! {
            <>
                <h2>
                    { "Fixed labels" }
                </h2>
                <Example code=include_str!("../examples/input_example2.rs")>
                    { include!("../examples/input_example2.rs") }
                </Example>
            </>
        }
    }

    fn example3(&self) -> Html {
        html! {
            <>
                <h2>
                    { "Floating labels" }
                </h2>
                <Example code=include_str!("../examples/input_example3.rs")>
                    { include!("../examples/input_example3.rs") }
                </Example>
            </>
        }
    }
}
