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

#[derive(Clone, Properties)]
pub struct Props {
    pub children: Children,
    pub class: Classes,
    pub onclick: Option<Callback<ClickEvent>>,
    pub color: Option<Color>,
    pub size: Option<Size>,
    pub variant: Option<Variant>,
    pub disabled: bool,
}

pub enum Msg {
    Click(ClickEvent),
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

    fn view(&self) -> Html {
        const BTN_CLASS: &str = "mui-btn";
        let mut class = self.props.class.clone();
        class.push(BTN_CLASS);
        if let Some(color) = self.props.color {
            class.push(&color.class(BTN_CLASS));
        }
        if let Some(size) = self.props.size {
            class.push(&size.class(BTN_CLASS));
        }
        if let Some(variant) = self.props.variant {
            class.push(&variant.class(BTN_CLASS));
        }
        let onclick = self.link.callback(Msg::Click);
        html! {
            <button ref=self.node_ref.clone()
                class=class
                onclick=onclick
                disabled=self.props.disabled>
                { self.props.children.render() }
            </button>
        }
    }
}
