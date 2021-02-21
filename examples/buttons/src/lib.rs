#![recursion_limit = "2048"]

use muicss_yew::button::{Button, Color, Size, Variant};
use wasm_bindgen::prelude::*;
use yew::prelude::*;

pub struct ButtonsExample;

impl Component for ButtonsExample {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        ButtonsExample
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <div>
                    <div>
                        <Button> { "Button" } </Button>
                        <Button color=Color::Primary> { "Button" } </Button>
                        <Button color=Color::Danger> { "Button" } </Button>
                        <Button color=Color::Accent> { "Button" } </Button>
                        <Button color=Color::Dark> { "Button" } </Button>
                    </div>
                    <div>
                        <Button disabled=true> { "Button" } </Button>
                        <Button color=Color::Primary disabled=true> { "Button" } </Button>
                        <Button color=Color::Danger disabled=true> { "Button" } </Button>
                        <Button color=Color::Accent disabled=true> { "Button" } </Button>
                        <Button color=Color::Dark disabled=true> { "Button" } </Button>
                    </div>
                </div>
                <div>
                    <div>
                        <Button variant=Variant::Flat> { "Button" } </Button>
                        <Button variant=Variant::Flat color=Color::Primary> { "Button" } </Button>
                        <Button variant=Variant::Flat color=Color::Danger> { "Button" } </Button>
                        <Button variant=Variant::Flat color=Color::Accent> { "Button" } </Button>
                        <Button variant=Variant::Flat color=Color::Dark> { "Button" } </Button>
                    </div>
                    <div>
                        <Button variant=Variant::Flat disabled=true> { "Button" } </Button>
                        <Button variant=Variant::Flat color=Color::Primary disabled=true> { "Button" } </Button>
                        <Button variant=Variant::Flat color=Color::Danger disabled=true> { "Button" } </Button>
                        <Button variant=Variant::Flat color=Color::Accent disabled=true> { "Button" } </Button>
                        <Button variant=Variant::Flat color=Color::Dark disabled=true> { "Button" } </Button>
                    </div>
                </div>
                <div>
                    <div>
                        <Button variant=Variant::Raised> { "Button" } </Button>
                        <Button variant=Variant::Raised color=Color::Primary> { "Button" } </Button>
                        <Button variant=Variant::Raised color=Color::Danger> { "Button" } </Button>
                        <Button variant=Variant::Raised color=Color::Accent> { "Button" } </Button>
                        <Button variant=Variant::Raised color=Color::Dark> { "Button" } </Button>
                    </div>
                    <div>
                        <Button variant=Variant::Raised disabled=true> { "Button" } </Button>
                        <Button variant=Variant::Raised color=Color::Primary disabled=true> { "Button" } </Button>
                        <Button variant=Variant::Raised color=Color::Danger disabled=true> { "Button" } </Button>
                        <Button variant=Variant::Raised color=Color::Accent disabled=true> { "Button" } </Button>
                        <Button variant=Variant::Raised color=Color::Dark disabled=true> { "Button" } </Button>
                    </div>
                </div>
                <div>
                    <div>
                        <Button variant=Variant::Fab> { "+" } </Button>
                        <Button variant=Variant::Fab color=Color::Primary> { "+" } </Button>
                        <Button variant=Variant::Fab color=Color::Danger> { "+" } </Button>
                        <Button variant=Variant::Fab color=Color::Accent> { "+" } </Button>
                        <Button variant=Variant::Fab color=Color::Dark> { "+" } </Button>
                    </div>
                    <div>
                        <Button variant=Variant::Fab disabled=true> { "+" } </Button>
                        <Button variant=Variant::Fab color=Color::Primary disabled=true> { "+" } </Button>
                        <Button variant=Variant::Fab color=Color::Danger disabled=true> { "+" } </Button>
                        <Button variant=Variant::Fab color=Color::Accent disabled=true> { "+" } </Button>
                        <Button variant=Variant::Fab color=Color::Dark disabled=true> { "+" } </Button>
                    </div>
                </div>
                <div>
                    <div>
                        <Button size=Size::Small color=Color::Primary> { "Button" } </Button>
                        <Button size=Size::Small color=Color::Primary variant=Variant::Flat> { "Button" } </Button>
                        <Button size=Size::Small color=Color::Primary variant=Variant::Raised> { "Button" } </Button>
                        <Button size=Size::Small color=Color::Primary variant=Variant::Fab> { "+" } </Button>
                    </div>
                    <div>
                        <Button color=Color::Primary> { "Button" } </Button>
                        <Button color=Color::Primary variant=Variant::Flat> { "Button" } </Button>
                        <Button color=Color::Primary variant=Variant::Raised> { "Button" } </Button>
                        <Button color=Color::Primary variant=Variant::Fab> { "+" } </Button>
                    </div>
                    <div>
                        <Button size=Size::Large color=Color::Primary> { "Button" } </Button>
                        <Button size=Size::Large color=Color::Primary variant=Variant::Flat> { "Button" } </Button>
                        <Button size=Size::Large color=Color::Primary variant=Variant::Raised> { "Button" } </Button>
                        <Button size=Size::Large color=Color::Primary variant=Variant::Fab> { "+" } </Button>
                    </div>
                </div>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<ButtonsExample>::new().mount_to_body();
}
