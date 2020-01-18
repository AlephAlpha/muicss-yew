#![recursion_limit = "512"]

mod button_sizes;
mod buttons;
mod fab_buttons;

use crate::{button_sizes::ButtonSizes, buttons::Buttons, fab_buttons::FabButtons};
use muicss_yew::components::button::{Size, Variant};
use yew::prelude::*;

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        App
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="mui-container">
                <h1> { "Buttons" } </h1>
                <div id="default-buttons">
                    <div class="mui-panel">
                        <h3> { "Default buttons" } </h3>
                        <Buttons />
                        <Buttons disabled=true />
                    </div>
                </div>
                <div id="flat-buttons">
                    <div class="mui-panel">
                        <h3> { "Flat buttons" } </h3>
                        <Buttons variant=Some(Variant::Flat)/>
                        <Buttons variant=Some(Variant::Flat) disabled=true />
                    </div>
                </div>
                <div id="raised-buttons">
                    <div class="mui-panel">
                        <h3> { "Raised buttons" } </h3>
                        <Buttons variant=Some(Variant::Raised)/>
                        <Buttons variant=Some(Variant::Raised) disabled=true />
                    </div>
                </div>
                <div id="floating-buttons">
                    <div class="mui-panel">
                        <h3> { "Floating Action Buttons" } </h3>
                        <FabButtons />
                        <FabButtons disabled=true />
                    </div>
                </div>
                <div id="button-sizes">
                    <div class="mui-panel">
                        <h3> { "Button sizes" } </h3>
                        <ButtonSizes size=Some(Size::Small) />
                        <ButtonSizes />
                        <ButtonSizes size=Some(Size::Large) />
                    </div>
                </div>
            </div>
        }
    }
}
