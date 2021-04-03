use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum InputType {
    Email,
    Password,
    Search,
    Tel,
    Text,
    Url,
}

impl Default for InputType {
    fn default() -> Self {
        Self::Text
    }
}

impl InputType {
    fn input_type(&self) -> &'static str {
        match self {
            InputType::Email => "email",
            InputType::Password => "password",
            InputType::Search => "search",
            InputType::Tel => "tel",
            InputType::Text => "text",
            InputType::Url => "url",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub onchange: Callback<ChangeData>,
    #[prop_or_default]
    pub input_type: InputType,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub invalid: bool,
    #[prop_or_default]
    pub floating_label: bool,
    #[prop_or_default]
    pub label: Option<String>,
    #[prop_or_default]
    pub value: String,
    #[prop_or_default]
    pub placeholder: String,
}

#[derive(Clone, Debug)]
pub struct Input {
    props: Props,
}

pub enum Msg {
    Change(ChangeData),
}

impl Component for Input {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Input { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        const TEXTFIELD_CLASS: &str = "mui-textfield";
        const FLOAT_LABEL_CLASS: &str = "mui-textfield--float-label";
        const INVALID_CLASS: &str = "mui--is-invalid";
        let class = self
            .props
            .class
            .clone()
            .extend(TEXTFIELD_CLASS)
            .extend(self.props.invalid.then(|| INVALID_CLASS))
            .extend(self.props.floating_label.then(|| FLOAT_LABEL_CLASS));

        let label = if let Some(label) = &self.props.label {
            html! {
                <label>
                    { label }
                </label>
            }
        } else {
            Html::default()
        };

        html! {
            <div class=class>
                <input type=self.props.input_type.input_type()
                    onchange=&self.props.onchange
                    disabled=self.props.disabled
                    placeholder=self.props.placeholder
                    value=self.props.value />
                { label }
            </div>
        }
    }
}
