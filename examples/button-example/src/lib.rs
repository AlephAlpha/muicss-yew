#![recursion_limit = "512"]

use muicss_yew::components::button::{self, Button};
use yew::prelude::*;

pub struct Example;

impl Component for Example {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Example
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <div>
                    <Button size=Some(button::Size::Small)
                        color=Some(button::Color::Primary)>
                        { "button" }
                    </Button>
                    <Button size=Some(button::Size::Small)
                        color=Some(button::Color::Primary)
                        variant=Some(button::Variant::Flat)>
                        { "button" }
                    </Button>
                    <Button size=Some(button::Size::Small)
                        color=Some(button::Color::Primary)
                        variant=Some(button::Variant::Raised)>
                        { "button" }
                    </Button>
                    <Button size=Some(button::Size::Small)
                        color=Some(button::Color::Primary)
                        variant=Some(button::Variant::Fab)>
                        { "+" }
                    </Button>
                </div>
                <div>
                    <Button color=Some(button::Color::Primary)>
                        { "button" }
                    </Button>
                    <Button color=Some(button::Color::Primary)
                        variant=Some(button::Variant::Flat)>
                        { "button" }
                    </Button>
                    <Button color=Some(button::Color::Primary)
                        variant=Some(button::Variant::Raised)>
                        { "button" }
                    </Button>
                    <Button color=Some(button::Color::Primary)
                        variant=Some(button::Variant::Fab)>
                        { "+" }
                    </Button>
                </div>
                <div>
                    <Button size=Some(button::Size::Large)
                        color=Some(button::Color::Primary)>
                        { "button" }
                    </Button>
                    <Button size=Some(button::Size::Large)
                        color=Some(button::Color::Primary)
                        variant=Some(button::Variant::Flat)>
                        { "button" }
                    </Button>
                    <Button size=Some(button::Size::Large)
                        color=Some(button::Color::Primary)
                        variant=Some(button::Variant::Raised)>
                        { "button" }
                    </Button>
                    <Button size=Some(button::Size::Large)
                        color=Some(button::Color::Primary)
                        variant=Some(button::Variant::Fab)>
                        { "+" }
                    </Button>
                </div>
            </div>
        }
    }
}
