use yew::prelude::*;

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

#[derive(PartialEq, Clone, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,
    #[prop_or_default]
    pub color: Option<Color>,
    #[prop_or_default]
    pub size: Option<Size>,
    #[prop_or_default]
    pub variant: Option<Variant>,
    #[prop_or_default]
    pub disabled: bool,
}

pub enum Msg {
    Click(MouseEvent),
}

pub struct Button {
    link: ComponentLink<Self>,
    props: Props,
    node_ref: NodeRef,
}

impl Component for Button {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Button {
            link,
            props,
            node_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click(e) => {
                if let Some(callback) = &self.props.onclick {
                    callback.emit(e);
                }
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
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

        let onclick = self.link.callback(Msg::Click);
        html! {
            <button ref=self.node_ref.clone()
                class=class
                onclick=onclick
                disabled=self.props.disabled>
                { self.props.children.clone() }
            </button>
        }
    }
}
