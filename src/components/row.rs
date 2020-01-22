use yew::prelude::*;

#[derive(Clone, Properties)]
pub struct Props {
    pub children: Children,
    pub class: Classes,
}

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

    fn view(&self) -> Html {
        const ROW_CLASS: &str = "mui-row";
        let mut class = self.props.class.clone();
        class.push(ROW_CLASS);
        html! {
            <div class=class>
                { self.props.children.render() }
            </div>
        }
    }
}
