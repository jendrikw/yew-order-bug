use yew::html::Env;
use yew::html::Html;
use yew::html::Renderable;
use yew::html::ShouldRender;
use yew::html::Component;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Props {
    pub str: String,
}

impl Default for Props {
    fn default() -> Props {
        Props {
            str: String::new(),
        }
    }
}

pub struct StringComponent {
    str: String,
}

impl Component<()> for StringComponent {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _context: &mut Env<(), Self>) -> Self {
        StringComponent {
            str: props.str,
        }
    }

    fn update(&mut self, _msg: (), _context: &mut Env<(), Self>) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties, _context: &mut Env<(), Self>) -> ShouldRender {
        self.str = props.str;
        true
    }
}

impl Renderable<(), StringComponent> for StringComponent {
    fn view(&self) -> Html<(), StringComponent> {
        html!{
            <>{ self.str.clone() }</>
        }
    }
}
