use yew::prelude::*;

#[derive(PartialEq, Clone, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

pub struct Appbar {
    props: Props,
}

impl Component for Appbar {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Appbar { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        const APPBAR_CLASS: &str = "mui-appbar";
        let class = self.props.class.clone().extend(APPBAR_CLASS);
        html! {
            <div class=class>
                { self.props.children.clone() }
            </div>
        }
    }
}
