use yew::prelude::*;

#[derive(Clone, Properties)]
pub struct Props {
    pub children: Children,
    pub class: Classes,
    pub fluid: bool,
}

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

    fn view(&self) -> Html {
        const CONTAINER_CLASS: &str = "mui-container";
        const CONTAINER_CLASS_FLUID: &str = "mui-container-fluid";
        let mut class = self.props.class.clone();
        class.push(if self.props.fluid {
            CONTAINER_CLASS_FLUID
        } else {
            CONTAINER_CLASS
        });
        html! {
            <div class=class>
                { self.props.children.render() }
            </div>
        }
    }
}
