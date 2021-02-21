use yew::prelude::*;

pub struct Divider;

impl Component for Divider {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Divider
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        const DIVIDER_CLASS: &str = "mui-divider";
        let class = Classes::from(DIVIDER_CLASS);
        html! {
            <div class=class></div>
        }
    }
}
