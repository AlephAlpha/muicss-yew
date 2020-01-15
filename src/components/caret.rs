use yew::prelude::*;

prop_enum! {
    Direction {
        Up => "up",
        Right => "right",
        Left => "left",
    }
}

#[derive(Clone, Properties)]
pub struct Props {
    pub direction: Option<Direction>,
}

pub struct Caret {
    props: Props,
}

impl Component for Caret {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Caret { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        const CARET_CLASS: &str = "mui-caret";
        let mut class = Classes::new();
        class.push(CARET_CLASS);
        if let Some(direction) = self.props.direction {
            class.push(&direction.class(CARET_CLASS));
        }
        html! {
            <span class=class></span>
        }
    }
}
