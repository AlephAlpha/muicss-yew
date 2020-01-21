use yew::prelude::*;

#[derive(Clone, Properties)]
pub struct Props {
    pub children: Children,
    pub class: Classes,
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

    fn view(&self) -> Html {
        const FORM_CLASS: &str = "mui-form";
        const FORM_CLASS_INLINE: &str = "mui-form--inline";
        let mut class = self.props.class.clone();
        class.push(FORM_CLASS);
        if self.props.inline {
            class.push(FORM_CLASS_INLINE);
        }
        html! {
            <div class=class>
                { self.props.children.render() }
            </div>
        }
    }
}
