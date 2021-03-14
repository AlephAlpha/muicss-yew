use crate::{
    button::{Color, Size, Variant},
    caret::{Caret, Direction},
    dropdown_item::DropdownItem,
};
use yew::prelude::*;
use yewtil::NeqAssign;

prop_enum! {
    Placement {
        Up => "up",
        Right => "right",
        Left => "left",
    }
}

impl Placement {
    fn direction(self) -> Direction {
        match self {
            Placement::Up => Direction::Up,
            Placement::Right => Direction::Right,
            Placement::Left => Direction::Left,
        }
    }
}

prop_enum! {
    Alignment {
        Right => "right",
        Bottom => "bottom",
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: ChildrenWithProps<DropdownItem>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub color: Option<Color>,
    #[prop_or_default]
    pub size: Option<Size>,
    #[prop_or_default]
    pub variant: Option<Variant>,
    #[prop_or_default]
    pub placement: Option<Placement>,
    #[prop_or_default]
    pub alignment: Option<Alignment>,
    #[prop_or_default]
    pub label: String,
    #[prop_or_default]
    pub disabled: bool,
}

pub struct Dropdown {
    props: Props,
}

impl Component for Dropdown {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Dropdown { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        const DROPDOWN_CLASS: &str = "mui-dropdown";
        let class = self
            .props
            .class
            .clone()
            .extend(DROPDOWN_CLASS)
            .extend(self.props.placement.map(|c| c.class(DROPDOWN_CLASS)));

        const BTN_CLASS: &str = "mui-btn";
        let button_class = Classes::from(BTN_CLASS)
            .extend(self.props.color.map(|c| c.class(BTN_CLASS)))
            .extend(self.props.size.map(|c| c.class(BTN_CLASS)))
            .extend(self.props.variant.map(|c| c.class(BTN_CLASS)));

        const MENU_CLASS: &str = "mui-dropdown__menu";
        let ui_class =
            Classes::from(MENU_CLASS).extend(self.props.alignment.map(|c| c.class(MENU_CLASS)));

        html! {
            <div class=class>
                <button class=button_class
                    disabled=self.props.disabled
                    data-mui-toggle="dropdown">
                    { &self.props.label }
                    <Caret direction=self.props.placement.map(Placement::direction) />
                </button>
                <ul class=ui_class>
                    { self.props.children.clone() }
                </ul>
            </div>
        }
    }
}
