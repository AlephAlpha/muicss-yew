#![recursion_limit = "512"]

use muicss_yew::components::form::Form;
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
                <h1> { "Form" } </h1>
                <div class="mui-panel">
                    <div id="example1">
                        <Form>
                            <legend>  { "Title" } </legend>
                            <div class="mui-textfield">
                                <input type="text" />
                            </div>
                            <button class="mui-btn"> { "Button" } </button>
                        </Form>
                    </div>
                </div>
                <div class="mui-panel">
                    <div id="example2">
                        <Form>
                            <fieldset>
                                <legend>  { "Fieldset1" } </legend>
                                <div class="mui-textfield">
                                    <input type="text" />
                                </div>
                                <div class="mui-textfield">
                                    <input type="text" />
                                </div>
                            </fieldset>
                            <fieldset>
                                <legend>  { "Fieldset2" } </legend>
                                <div class="mui-textfield">
                                    <input type="text" />
                                </div>
                                <div class="mui-textfield">
                                    <input type="text" />
                                </div>
                            </fieldset>
                            <button class="mui-btn"> { "Button" } </button>
                        </Form>
                    </div>
                </div>
                <div class="mui-panel">
                    <div id="example3">
                        <Form inline=true>
                            <legend>  { "Title" } </legend>
                            <div class="mui-textfield">
                                <input type="text" />
                            </div>
                            <button class="mui-btn"> { "Button" } </button>
                        </Form>
                    </div>
                </div>
                <div class="mui-panel">
                    <div id="example4">
                        <Form inline=true>
                            <fieldset>
                                <legend>  { "Fieldset1" } </legend>
                                <div class="mui-textfield">
                                    <input type="text" />
                                </div>
                                <div class="mui-textfield">
                                    <input type="text" />
                                </div>
                            </fieldset>
                            <fieldset>
                                <legend>  { "Fieldset2" } </legend>
                                <div class="mui-textfield">
                                    <input type="text" />
                                </div>
                                <div class="mui-textfield">
                                    <input type="text" />
                                </div>
                            </fieldset>
                            <button class="mui-btn"> { "Button" } </button>
                        </Form>
                    </div>
                </div>
            </div>
        }
    }
}
