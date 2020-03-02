use muicss_yew::components::button::{Button, Color, Size, Variant};
use yew::prelude::*;

#[derive(Clone, Properties)]
pub struct Props {
    #[prop_or_default]
    pub size: Option<Size>,
    #[prop_or_default]
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
                <Button variant=Variant::Fab
                    size=self.props.size
                    disabled=self.props.disabled>
                    { "+" }
                </Button>
                <Button color=Color::Primary
                    variant=Variant::Fab
                    size=self.props.size
                    disabled=self.props.disabled>
                    { "+" }
                </Button>
                <Button color=Color::Danger
                    variant=Variant::Fab
                    size=self.props.size
                    disabled=self.props.disabled>
                    { "+" }
                </Button>
                <Button color=Color::Dark
                    variant=Variant::Fab
                    size=self.props.size
                    disabled=self.props.disabled>
                    { "+" }
                </Button>
                <Button color=Color::Accent
                    variant=Variant::Fab
                    size=self.props.size
                    disabled=self.props.disabled>
                    { "+" }
                </Button>
            </div>
        }
    }
}
