#![recursion_limit = "1024"]

use muicss_yew::{
    button::Color,
    dropdown::{Alignment, Dropdown, Placement},
    dropdown_item::DropdownItem,
};
use wasm_bindgen::prelude::*;
use yew::prelude::*;

pub struct DropdownsExample;

impl Component for DropdownsExample {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        DropdownsExample
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
                    <Dropdown label="Dropdown" color=Color::Primary>
                        <DropdownItem link="#">
                            { "Option 1" }
                        </DropdownItem>
                        <DropdownItem>
                            { "Option 2" }
                        </DropdownItem>
                        <DropdownItem>
                            { "Option 3" }
                        </DropdownItem>
                        <DropdownItem>
                            { "Option 4" }
                        </DropdownItem>
                    </Dropdown>
                </div>
                <div>
                    <Dropdown label="Dropdown (right aligned)" color=Color::Primary alignment=Alignment::Right>
                        <DropdownItem link="#">
                            { "Option 1" }
                        </DropdownItem>
                        <DropdownItem>
                            { "Option 2" }
                        </DropdownItem>
                        <DropdownItem>
                            { "Option 3" }
                        </DropdownItem>
                        <DropdownItem>
                            { "Option 4" }
                        </DropdownItem>
                    </Dropdown>
                </div>
                <div>
                    <Dropdown label="Dropup" color=Color::Primary placement=Placement::Up>
                        <DropdownItem link="#">
                            { "Option 1" }
                        </DropdownItem>
                        <DropdownItem>
                            { "Option 2" }
                        </DropdownItem>
                        <DropdownItem>
                            { "Option 3" }
                        </DropdownItem>
                        <DropdownItem>
                            { "Option 4" }
                        </DropdownItem>
                    </Dropdown>
                </div>
                <div>
                    <Dropdown label="Dropup (right aligned)" color=Color::Primary placement=Placement::Up alignment=Alignment::Right>
                        <DropdownItem link="#">
                            { "Option 1" }
                        </DropdownItem>
                        <DropdownItem>
                            { "Option 2" }
                        </DropdownItem>
                        <DropdownItem>
                            { "Option 3" }
                        </DropdownItem>
                        <DropdownItem>
                            { "Option 4" }
                        </DropdownItem>
                    </Dropdown>
                </div>
                <div>
                    <Dropdown label="Dropright" color=Color::Primary placement=Placement::Right>
                        <DropdownItem link="#">
                            { "Option 1" }
                        </DropdownItem>
                        <DropdownItem>
                            { "Option 2" }
                        </DropdownItem>
                        <DropdownItem>
                            { "Option 3" }
                        </DropdownItem>
                        <DropdownItem>
                            { "Option 4" }
                        </DropdownItem>
                    </Dropdown>
                </div>
                <div>
                    <Dropdown label="Dropright (bottom aligned)" color=Color::Primary placement=Placement::Right alignment=Alignment::Bottom>
                        <DropdownItem link="#">
                            { "Option 1" }
                        </DropdownItem>
                        <DropdownItem>
                            { "Option 2" }
                        </DropdownItem>
                        <DropdownItem>
                            { "Option 3" }
                        </DropdownItem>
                        <DropdownItem>
                            { "Option 4" }
                        </DropdownItem>
                    </Dropdown>
                </div>
                <div>
                    <Dropdown label="Dropleft" color=Color::Primary placement=Placement::Left>
                        <DropdownItem link="#">
                            { "Option 1" }
                        </DropdownItem>
                        <DropdownItem>
                            { "Option 2" }
                        </DropdownItem>
                        <DropdownItem>
                            { "Option 3" }
                        </DropdownItem>
                        <DropdownItem>
                            { "Option 4" }
                        </DropdownItem>
                    </Dropdown>
                </div>
                <div>
                    <Dropdown label="Dropleft (bottom aligned)" color=Color::Primary placement=Placement::Left alignment=Alignment::Bottom>
                        <DropdownItem link="#">
                            { "Option 1" }
                        </DropdownItem>
                        <DropdownItem>
                            { "Option 2" }
                        </DropdownItem>
                        <DropdownItem>
                            { "Option 3" }
                        </DropdownItem>
                        <DropdownItem>
                            { "Option 4" }
                        </DropdownItem>
                    </Dropdown>
                </div>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<DropdownsExample>::new().mount_to_body();
}
