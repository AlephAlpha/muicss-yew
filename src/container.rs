use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub fluid: bool,
}

#[derive(Clone, Debug)]
pub struct Container {
    props: Props,
}

impl Component for Container {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Container { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        const CONTAINER_CLASS: &str = "mui-container";
        const CONTAINER_CLASS_FLUID: &str = "mui-container-fluid";
        let class = self.props.class.clone().extend(if self.props.fluid {
            CONTAINER_CLASS_FLUID
        } else {
            CONTAINER_CLASS
        });
        html! {
            <div class=class>
                { self.props.children.clone() }
            </div>
        }
    }
}
