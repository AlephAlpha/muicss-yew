use muicss_yew::{appbar::Appbar, container::Container};
use yew::prelude::*;
use yew_feather::github::Github;

#[derive(Clone, Debug)]
pub struct Header;

impl Component for Header {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Header
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <header>
                <Appbar>
                    <Container fluid=true>
                        <table width="100%">
                            <tbody>
                                <tr class="mui--appbar-height">
                                    <td class="mui--text-headline">
                                        { "MUICSS-Yew" }
                                    </td>
                                    <td class="mui--text-right">
                                        <a href="https://github.com/AlephAlpha/muicss-yew">
                                            <Github color="#FFF" />
                                        </a>
                                    </td>
                                </tr>
                            </tbody>
                        </table>
                    </Container>
                </Appbar>
            </header>
        }
    }
}
