use muicss_yew::components::appbar::Appbar;
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
            <Appbar>
                <table width="100%">
                    <tbody>
                        <tr style="vertical-align:middle;">
                            <td class="mui--appbar-height"> { "Left Side" } </td>
                            <td class="mui--appbar-height" align="right"> { "Right Side" } </td>
                        </tr>
                    </tbody>
                </table>
            </Appbar>
        }
    }
}
