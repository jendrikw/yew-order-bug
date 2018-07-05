#![feature(associated_type_defaults)]
#![recursion_limit="128"]

#[macro_use]
extern crate yew;
extern crate stdweb;

mod string_component;
mod root_component;

use root_component::RootComponent;
use yew::prelude::App;

fn main() {
    yew::initialize();
    App::<(), RootComponent>::new(()).mount_to_body();
    yew::run_loop();
}
