use muicss_yew::components::button::{Button, Color, Size, Variant};
use yew::prelude::*;

#[derive(Clone, Properties)]
pub struct Props {
    pub size: Option<Size>,
    pub disabled: bool,
}

pub struct FabButtons {
    props: Props,
}

impl Component for FabButtons {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        FabButtons { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <Button variant=Some(Variant::Fab)
                    size=self.props.size
                    disabled=self.props.disabled>
                    { "+" }
                </Button>
                <Button color=Some(Color::Primary)
                    variant=Some(Variant::Fab)
                    size=self.props.size
                    disabled=self.props.disabled>
                    { "+" }
                </Button>
                <Button color=Some(Color::Danger)
                    variant=Some(Variant::Fab)
                    size=self.props.size
                    disabled=self.props.disabled>
                    { "+" }
                </Button>
                <Button color=Some(Color::Dark)
                    variant=Some(Variant::Fab)
                    size=self.props.size
                    disabled=self.props.disabled>
                    { "+" }
                </Button>
                <Button color=Some(Color::Accent)
                    variant=Some(Variant::Fab)
                    size=self.props.size
                    disabled=self.props.disabled>
                    { "+" }
                </Button>
            </div>
        }
    }
}