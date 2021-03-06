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
pub struct Row {
    props: Props,
}

impl Component for Row {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Row { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        const ROW_CLASS: &str = "mui-row";
        let class = self.props.class.clone().extend(ROW_CLASS);
        html! {
            <div class=class>
                { self.props.children.clone() }
            </div>
        }
    }
}
