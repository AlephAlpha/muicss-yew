use crate::components::example::Example;
use muicss_yew::caret::{Caret, Direction};
use yew::prelude::*;
use yew_prism::Prism;

#[derive(Clone, Debug)]
pub struct CaretExamples;

impl Component for CaretExamples {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        CaretExamples
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
            </>
        }
    }
}

impl CaretExamples {
    const IMPORT: &'static str = r#"use muicss_yew::caret::{Caret, Direction};"#;

    fn introduction(&self) -> Html {
        html! {
            <>
                <h1>
                    { "Caret" }
                </h1>
                <Prism code=Self::IMPORT language="rust" />
                <p>
                    { "MUI's website doesn't have a page for this component." }
                </p>
            </>
        }
    }

    fn example1(&self) -> Html {
        html! {
            <>
                <Example code=include_str!("../examples/caret_example1.rs")>
                    { include!("../examples/caret_example1.rs") }
                </Example>
            </>
        }
    }
}
