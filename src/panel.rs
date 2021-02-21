use yew::prelude::*;

#[derive(Clone, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

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

    fn view(&self) -> Html {
        const PANEL_CLASS: &str = "mui-panel";
        let mut class = self.props.class.clone();
        class.push(PANEL_CLASS);
        html! {
            <div class=class>
                { self.props.children.render() }
            </div>
        }
    }
}
