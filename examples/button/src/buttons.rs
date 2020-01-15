use muicss_yew::components::button::{Button, Color, Size, Variant};
use yew::prelude::*;

#[derive(Clone, Properties)]
pub struct Props {
    pub size: Option<Size>,
    pub variant: Option<Variant>,
    pub disabled: bool,
}

pub struct Buttons {
    props: Props,
}

impl Component for Buttons {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Buttons { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <Button variant=self.props.variant
                    size=self.props.size
                    disabled=self.props.disabled>
                    { "Button" }
                </Button>
                <Button color=Some(Color::Primary)
                    variant=self.props.variant
                    size=self.props.size
                    disabled=self.props.disabled>
                    { "Button" }
                </Button>
                <Button color=Some(Color::Danger)
                    variant=self.props.variant
                    size=self.props.size
                    disabled=self.props.disabled>
                    { "Button" }
                </Button>
                <Button color=Some(Color::Dark)
                    variant=self.props.variant
                    size=self.props.size
                    disabled=self.props.disabled>
                    { "Button" }
                </Button>
                <Button color=Some(Color::Accent)
                    variant=self.props.variant
                    size=self.props.size
                    disabled=self.props.disabled>
                    { "Button" }
                </Button>
            </div>
        }
    }
}
