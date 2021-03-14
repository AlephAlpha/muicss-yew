use muicss_yew::panel::Panel;
use yew::prelude::*;
use yew_prism::Prism;
use yewtil::NeqAssign;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub code: String,
    #[prop_or_default]
    pub children: Children,
}

#[derive(Clone, Debug)]
pub struct Example {
    props: Props,
}

impl Component for Example {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Example { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        html! {
            <Panel>
                { self.props.children.clone() }
                <Prism code=self.props.code.clone() language="rust" />
            </Panel>
        }
    }
}
