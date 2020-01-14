use yew::prelude::*;

#[derive(Copy, Clone)]
pub enum Direction {
    Up,
    Right,
    Left,
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
        let mut classes = Classes::new();
        classes.push(CARET_CLASS);
        if let Some(direction) = self.props.direction {
            classes.push(&format!(
                "{}--{}",
                CARET_CLASS,
                match direction {
                    Direction::Up => "up",
                    Direction::Right => "right",
                    Direction::Left => "left",
                }
            ));
        }
        html! {
            <span class=classes></span>
        }
    }
}
