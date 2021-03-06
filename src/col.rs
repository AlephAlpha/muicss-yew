use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub xs: Option<u8>,
    #[prop_or_default]
    pub sm: Option<u8>,
    #[prop_or_default]
    pub md: Option<u8>,
    #[prop_or_default]
    pub lg: Option<u8>,
    #[prop_or_default]
    pub xl: Option<u8>,
    #[prop_or_default]
    pub xs_offset: Option<u8>,
    #[prop_or_default]
    pub sm_offset: Option<u8>,
    #[prop_or_default]
    pub md_offset: Option<u8>,
    #[prop_or_default]
    pub lg_offset: Option<u8>,
    #[prop_or_default]
    pub xl_offset: Option<u8>,
}

impl Props {
    fn responsive(&self) -> [(&str, Option<u8>, Option<u8>); 5] {
        [
            ("xs", self.xs, self.xs_offset),
            ("sm", self.sm, self.sm_offset),
            ("md", self.md, self.md_offset),
            ("lg", self.lg, self.lg_offset),
            ("xl", self.xl, self.xl_offset),
        ]
    }
}

#[derive(Clone, Debug)]
pub struct Col {
    props: Props,
}

impl Component for Col {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Col { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        const COL_CLASS: &str = "mui-col";
        let mut class = self.props.class.clone().extend(COL_CLASS);
        for (prefix, value, offset_value) in &self.props.responsive() {
            if let Some(value) = value {
                class.push(&format!("{}-{}-{}", COL_CLASS, prefix, value));
            }
            if let Some(value) = offset_value {
                class.push(&format!("{}-offset-{}-{}", COL_CLASS, prefix, value));
            }
        }
        html! {
            <div class=class>
                { self.props.children.clone() }
            </div>
        }
    }
}
