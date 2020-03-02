use yew::prelude::*;

#[derive(Clone, Properties)]
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

    fn view(&self) -> Html {
        const APPBAR_CLASS: &str = "mui-appbar";
        let mut class = self.props.class.clone();
        class.push(APPBAR_CLASS);
        html! {
            <div class=class>
                { self.props.children.render() }
            </div>
        }
    }
}
