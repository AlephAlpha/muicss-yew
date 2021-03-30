use crate::components::example::Example;
use muicss_yew::panel::Panel;
use yew::prelude::*;
use yew_prism::Prism;

#[derive(Clone, Debug)]
pub struct PanelExamples;

impl Component for PanelExamples {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        PanelExamples
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

impl PanelExamples {
    const IMPORT: &'static str = r#"use muicss_yew::panel::Panel;"#;

    fn introduction(&self) -> Html {
        html! {
            <>
                <h1>
                    { "Panels" }
                </h1>
                <Prism code=Self::IMPORT language="rust" />
                <p>
                    { "See " }
                    <a href="https://www.muicss.com/docs/v1/react/panels">
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
                <Example code=include_str!("../examples/panel_example1.rs")>
                    { include!("../examples/panel_example1.rs") }
                </Example>
            </>
        }
    }
}
