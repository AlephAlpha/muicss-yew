use std::time::Duration;
use stdweb::web::{event::IMouseEvent, HtmlElement, IHtmlElement};
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
    MouseDown(MouseDownEvent),
    MouseUp(MouseUpEvent),
    MouseLeave(MouseLeaveEvent),
    Animate,
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

pub struct Button {
    link: ComponentLink<Self>,
    props: Props,
    ripple_style: Option<RippleStyle>,
    ripple_is_visible: bool,
    ripple_is_animating: bool,
    ripple_animation_timeout: TimeoutService,
    ripple_animation_task: Option<TimeoutTask>,
    node_ref: NodeRef,
    ripple_ref: NodeRef,
}

impl Button {
    fn show_ripple(&mut self, event: MouseDownEvent) {
        if let Some(element) = self.node_ref.try_into::<HtmlElement>() {
            let offset = (event.offset_x(), event.offset_y());
            let size = (element.offset_width(), element.offset_height());
            let radius = ((size.0 * size.0 + size.1 * size.1) as f64).sqrt();
            self.ripple_style = Some(RippleStyle { offset, radius });
            self.ripple_is_visible = true;
            self.ripple_is_animating = false;
            let animation = self.link.callback(|_| Msg::Animate);
            let task = self
                .ripple_animation_timeout
                .spawn(Duration::default(), animation);
            self.ripple_animation_task = Some(task);
        }
    }

    fn hide_ripple(&mut self) {
        self.ripple_style = None;
        self.ripple_is_visible = false;
    }
}

impl Component for Button {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Button {
            link,
            props,
            ripple_style: None,
            ripple_is_visible: false,
            ripple_is_animating: true,
            ripple_animation_timeout: TimeoutService::new(),
            ripple_animation_task: None,
            node_ref: NodeRef::default(),
            ripple_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click(e) => {
                if let Some(callback) = &self.props.onclick {
                    callback.emit(e);
                }
            }
            Msg::MouseDown(e) => self.show_ripple(e),
            Msg::MouseUp(_e) => self.hide_ripple(),
            Msg::MouseLeave(_e) => self.hide_ripple(),
            Msg::Animate => self.ripple_is_animating = true,
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

        let mut ripple_class = Classes::from("mui-ripple");
        if self.ripple_is_visible {
            ripple_class.push("mui--is-visible");
        }
        if self.ripple_is_animating {
            ripple_class.push("mui--is-animating");
        }

        let onclick = self.link.callback(Msg::Click);
        let onmousedown = self.link.callback(Msg::MouseDown);
        let onmouseup = self.link.callback(Msg::MouseUp);
        let onmouseleave = self.link.callback(Msg::MouseLeave);

        let ripple_style = if let Some(style) = &self.ripple_style {
            style.to_style()
        } else {
            String::new()
        };

        html! {
            <button ref=self.node_ref.clone()
                class=class
                onclick=onclick
                onmousedown=onmousedown
                onmouseup=onmouseup
                onmouseleave=onmouseleave
                disabled=self.props.disabled>
                { self.props.children.render() }
                <span class="mui-btn__ripple-container">
                    <span ref=self.ripple_ref.clone()
                        class=ripple_class
                        style=ripple_style>
                    </span>
                </span>
            </button>
        }
    }
}
