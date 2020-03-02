use yew::prelude::*;

#[derive(Clone, Properties)]
pub struct Props {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub onchange: Option<Callback<ChangeData>>,
    #[prop_or_default]
    pub checked: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub label: String,
    #[prop_or_default]
    pub value: Option<String>,
}

pub struct Checkbox {
    link: ComponentLink<Self>,
    props: Props,
}

pub enum Msg {
    Change(ChangeData),
}

impl Component for Checkbox {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Checkbox { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Change(e) => {
                self.props.checked = !self.props.checked;
                if let Some(callback) = &self.props.onchange {
                    callback.emit(e);
                }
            }
        }
        true
    }

    fn view(&self) -> Html {
        const CHECKBOX_CLASS: &str = "mui-checkbox";
        let mut class = self.props.class.clone();
        class.push(CHECKBOX_CLASS);
        let onchange = self.link.callback(Msg::Change);
        html! {
            <div class=class>
                <label>
                    <input type="checkbox"
                        checked=self.props.checked
                        onchange=onchange
                        disabled=self.props.disabled
                        value=self.props.value.as_deref().unwrap_or("on") />
                    { &self.props.label }
                </label>
            </div>
        }
    }
}
