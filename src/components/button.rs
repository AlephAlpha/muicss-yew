use yew::prelude::*;

#[derive(Copy, Clone)]
pub enum Color {
    Primary,
    Dark,
    Danger,
    Accent,
}

#[derive(Copy, Clone)]
pub enum Size {
    Small,
    Large,
}

#[derive(Copy, Clone)]
pub enum Variant {
    Flat,
    Raised,
    Fab,
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
        let onclick = self.link.callback(Msg::Click);
        if let Some(color) = self.props.color {
            class.push(&format!(
                "{}--{}",
                BTN_CLASS,
                match color {
                    Color::Primary => "primary",
                    Color::Dark => "dark",
                    Color::Danger => "danger",
                    Color::Accent => "accent",
                }
            ));
        }
        if let Some(size) = self.props.size {
            class.push(&format!(
                "{}--{}",
                BTN_CLASS,
                match size {
                    Size::Small => "small",
                    Size::Large => "large",
                }
            ));
        }
        if let Some(variant) = self.props.variant {
            class.push(&format!(
                "{}--{}",
                BTN_CLASS,
                match variant {
                    Variant::Flat => "flat",
                    Variant::Raised => "raised",
                    Variant::Fab => "fab",
                }
            ));
        }
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
