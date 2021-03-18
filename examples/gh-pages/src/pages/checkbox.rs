use crate::components::example::Example;
use muicss_yew::checkbox::Checkbox;
use yew::prelude::*;
use yew_prism::Prism;

#[derive(Clone, Debug)]
pub struct CheckboxExamples;

impl Component for CheckboxExamples {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        CheckboxExamples
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

impl CheckboxExamples {
    const IMPORT: &'static str = r#"use muicss_yew::checkbox::Checkbox;"#;

    fn introduction(&self) -> Html {
        html! {
            <>
                <h1>
                    { "Checkbox" }
                </h1>
                <Prism code=Self::IMPORT language="rust" />
                <p>
                    { "See " }
                    <a href="https://www.muicss.com/docs/v1/react/checkbox">
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
                <Example code=include_str!("../examples/checkbox_example1.rs")>
                    { include!("../examples/checkbox_example1.rs") }
                </Example>
            </>
        }
    }
}
