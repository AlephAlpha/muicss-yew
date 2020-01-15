use muicss_yew::components::button::{Button, Color, Size, Variant};
use yew::prelude::*;

#[derive(Clone, Properties)]
pub struct Props {
    pub size: Option<Size>,
}

pub struct ButtonSizes {
    props: Props,
}

impl Component for ButtonSizes {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        ButtonSizes { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <Button color=Some(Color::Primary)
                    size=self.props.size>
                    { "Button" }
                </Button>
                <Button color=Some(Color::Primary)
                    variant=Some(Variant::Flat)
                    size=self.props.size>
                    { "Button" }
                </Button>
                <Button color=Some(Color::Primary)
                    variant=Some(Variant::Raised)
                    size=self.props.size>
                    { "Button" }
                </Button>
                <Button color=Some(Color::Primary)
                    variant=Some(Variant::Fab)
                    size=self.props.size>
                    { "+" }
                </Button>
            </div>
        }
    }
}
