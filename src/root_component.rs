use yew::html::ShouldRender;
use yew::html::{Component, Env, Html, Renderable};

use string_component::StringComponent;

pub struct RootComponent {}

impl Component<()> for RootComponent {
    type Message = ();
    type Properties = ();

    fn create(_props: (), _context: &mut Env<(), Self>) -> Self {
        RootComponent {}
    }

    fn update(&mut self, _msg: (), _context: &mut Env<(), Self>) -> ShouldRender {
        true
    }
}

impl Renderable<(), RootComponent> for RootComponent {
    fn view(&self) -> Html<(), RootComponent> {
        html! {
            <div>
                <StringComponent: str="test", />
                <h1>{ "content" }</h1>
            </div>
        }
    }
}
