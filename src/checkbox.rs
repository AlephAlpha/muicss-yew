use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub onchange: Callback<ChangeData>,
    #[prop_or_default]
    pub checked: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub value: Option<String>,
}

#[derive(Clone, Debug)]
pub struct Checkbox {
    props: Props,
}

impl Component for Checkbox {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Checkbox { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        const CHECKBOX_CLASS: &str = "mui-checkbox";
        let class = self.props.class.clone().extend(CHECKBOX_CLASS);
        html! {
            <div class=class>
                <label>
                    <input type="checkbox"
                        checked=self.props.checked
                        onchange=&self.props.onchange
                        disabled=self.props.disabled
                        value=self.props.value.as_deref().unwrap_or("on") />
                    { self.props.children.clone() }
                </label>
            </div>
        }
    }
}
