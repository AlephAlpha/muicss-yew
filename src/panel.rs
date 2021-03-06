use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[derive(Clone, Debug)]
pub struct Panel {
    props: Props,
}

impl Component for Panel {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Panel { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        const PANEL_CLASS: &str = "mui-panel";
        let class = self.props.class.clone().extend(PANEL_CLASS);
        html! {
            <div class=class>
                { self.props.children.clone() }
            </div>
        }
    }
}
