use std::time::Duration;
use stdweb::web::{
    event::{IMouseEvent, ITouchEvent},
    HtmlElement, IHtmlElement,
};
use yew::{
    prelude::*,
    services::{timeout::TimeoutTask, TimeoutService},
};

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
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub onclick: Option<Callback<ClickEvent>>,
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
    Click(ClickEvent),
    MouseDown(MouseDownEvent),
    TouchStart(TouchStart),
    TouchEnd,
    ShowRipple,
    HideRipple,
}

struct RippleStyle {
    offset: (f64, f64),
    radius: f64,
}

impl RippleStyle {
    fn to_style(&self) -> String {
        let top = (self.offset.1 - self.radius).round() as i32;
        let left = (self.offset.0 - self.radius).round() as i32;
        let diameter = 2 * self.radius.ceil() as i32;
        format!(
            "top: {}px; left: {}px; width: {}px; height: {}px;",
            top, left, diameter, diameter
        )
    }
}

pub struct Ripple {
    style: Option<RippleStyle>,
    is_visible: bool,
    is_animating: bool,
    timeout: TimeoutService,
    task: Option<TimeoutTask>,
}

impl Ripple {
    fn new() -> Self {
        Ripple {
            style: None,
            is_visible: false,
            is_animating: true,
            timeout: TimeoutService::new(),
            task: None,
        }
    }

    fn class(&self) -> Classes {
        let mut class = Classes::from("mui-ripple");
        if self.is_visible {
            class.push("mui--is-visible");
        }
        if self.is_animating {
            class.push("mui--is-animating");
        }
        class
    }

    fn style(&self) -> String {
        if let Some(style) = &self.style {
            style.to_style()
        } else {
            String::new()
        }
    }
}

impl Renderable for Ripple {
    fn render(&self) -> Html {
        html! {
            <span class="mui-btn__ripple-container">
                <span class=self.class()
                    style=self.style()>
                </span>
            </span>
        }
    }
}

pub struct Button {
    link: ComponentLink<Self>,
    props: Props,
    ripple: Ripple,
    node_ref: NodeRef,
}

impl Button {
    fn show_ripple(&mut self, offset: (f64, f64)) {
        if let Some(element) = self.node_ref.cast::<HtmlElement>() {
            let size = (element.offset_width(), element.offset_height());
            let radius = ((size.0 * size.0 + size.1 * size.1) as f64).sqrt();
            self.ripple.style = Some(RippleStyle { offset, radius });
            self.ripple.is_visible = true;
            self.ripple.is_animating = false;
            let timeout = self.link.callback(|_| Msg::ShowRipple);
            let task = self.ripple.timeout.spawn(Duration::default(), timeout);
            self.ripple.task = Some(task);
        }
    }

    fn hide_ripple(&mut self) {
        let timeout = self.link.callback(|_| Msg::HideRipple);
        let task = self
            .ripple
            .timeout
            .spawn(Duration::from_millis(10), timeout);
        self.ripple.task = Some(task);
    }
}

impl Component for Button {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Button {
            link,
            props,
            ripple: Ripple::new(),
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
            Msg::MouseDown(e) => {
                let offset = (e.offset_x(), e.offset_y());
                self.show_ripple(offset);
            }
            Msg::TouchStart(e) => {
                if let Some(element) = self.node_ref.cast::<HtmlElement>() {
                    let touch = &e.touches()[0];
                    let rect = element.get_bounding_client_rect();
                    let offset = (
                        touch.client_x() - rect.get_left(),
                        touch.client_y() - rect.get_top(),
                    );
                    self.show_ripple(offset);
                }
            }
            Msg::TouchEnd => self.hide_ripple(),
            Msg::ShowRipple => self.ripple.is_animating = true,
            Msg::HideRipple => self.ripple.is_visible = false,
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
        let onmousedown = self.link.callback(Msg::MouseDown);
        let onmouseup = self.link.callback(|_| Msg::TouchEnd);
        let onmouseleave = self.link.callback(|_| Msg::TouchEnd);
        let ontouchstart = self.link.callback(Msg::TouchStart);
        let ontouchend = self.link.callback(|_| Msg::TouchEnd);
        html! {
            <button ref=self.node_ref.clone()
                class=class
                onclick=onclick
                onmousedown=onmousedown
                onmouseup=onmouseup
                onmouseleave=onmouseleave
                ontouchstart=ontouchstart
                ontouchend=ontouchend
                disabled=self.props.disabled>
                { self.props.children.render() }
                { self.ripple.render() }
            </button>
        }
    }
}
