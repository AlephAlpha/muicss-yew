use yew::prelude::*;
use yewtil::NeqAssign;

prop_enum! {
    Color {
        Primary => "primary",
        Danger => "danger",
        Dark => "dark",
        Accent => "accent",
    }
}

prop_enum! {
    Size {
        Small => "small",
        Large => "large",
    }
}

prop_enum! {
    Variant {
        Flat => "flat",
        Raised => "raised",
        Fab => "fab",
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub color: Option<Color>,
    #[prop_or_default]
    pub size: Option<Size>,
    #[prop_or_default]
    pub variant: Option<Variant>,
    #[prop_or_default]
    pub disabled: bool,
}

#[derive(Clone, Debug)]
pub struct Button {
    props: Props,
    node_ref: NodeRef,
}

impl Component for Button {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Button {
            props,
            node_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        const BTN_CLASS: &str = "mui-btn";
        let class = self
            .props
            .class
            .clone()
            .extend(BTN_CLASS)
            .extend(self.props.color.map(|c| c.class(BTN_CLASS)))
            .extend(self.props.size.map(|c| c.class(BTN_CLASS)))
            .extend(self.props.variant.map(|c| c.class(BTN_CLASS)));

        html! {
            <button ref=self.node_ref.clone()
                class=class
                onclick=&self.props.onclick
                disabled=self.props.disabled>
                { self.props.children.clone() }
            </button>
        }
    }
}
