use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub inline: bool,
}

pub struct Form {
    props: Props,
}

impl Component for Form {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Form { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        const FORM_CLASS: &str = "mui-form";
        const FORM_CLASS_INLINE: &str = "mui-form--inline";
        let class = self.props.class.clone().extend(if self.props.inline {
            FORM_CLASS_INLINE
        } else {
            FORM_CLASS
        });
        html! {
            <form class=class>
                { self.props.children.clone() }
            </form>
        }
    }
}
