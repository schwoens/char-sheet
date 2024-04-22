use crate::components::sheet::Sheet;
use yew::prelude::*;

pub mod components;

#[derive(PartialEq, Clone)]
pub struct Character {
    pub name: String,
    pub level: u8,
    pub class: String,
    pub race: String,
}

impl Default for Character {
    fn default() -> Self {
        Self {
            name: String::new(),
            level: 1,
            class: String::new(),
            race: String::new(),
        }
    }
}

#[function_component]
fn App() -> Html {
    html! {
        <Sheet/>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
