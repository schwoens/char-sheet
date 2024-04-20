use std::ops::Deref;
use implicit_clone::unsync::IString;
use yew::{html::IntoPropValue, prelude::*, virtual_dom::VNode};

#[derive(Properties, PartialEq)]
pub struct SheetProps {
    pub character: Character 
}

#[derive(Properties, PartialEq)]
pub struct InputProps<T>
where 
    T: Clone + PartialEq + IntoPropValue<VNode>
{
    pub value: T,
    pub label: String,
}

#[derive(PartialEq)]
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
fn Sheet(props: &SheetProps) -> Html {
    let char = &props.character;

    html! {
        <div class={classes!("flex", "flex-column")}>
            <Input<String> 
                value={char.name.clone()} 
                label={"Name: "} 
            />
            <span>{"Level: "}{char.level.clone()}</span>
        </div>
    }
}

#[function_component]
fn Input<T>(props: &InputProps<T>) -> Html 
where T: 
    PartialEq +
    Clone + 
    IntoPropValue<VNode> +
    IntoPropValue<Option<IString>> +
{
    let active = use_state(|| false);
    let value = &props.value.clone();

    let onclick = {
        let active = active.clone();
        Callback::from(move |_: MouseEvent| {
            active.set(!active.clone().deref());
        })
    };

    let onfocusout = {
        let active = active.clone();
        let value = value.clone();
        Callback::from(move |e: FocusEvent| {
            active.set(!active.clone().deref());
            log::info!("value: {:?}", e.target().unwrap());
        })
    };

    html! {
        <div>
            <label>{ props.label.clone() }</label>
            if *active.deref() {
                <input 
                    type="text" 
                    value={props.value.clone()}
                    onfocusout={onfocusout}
                />
            } else {
                <span onclick={onclick} class={classes!("input")}>
                    { props.value.clone() }
                </span>
            }
        </div>
    }
}

#[function_component]
fn App() -> Html {
    html! {
        <Sheet character={Character::default()} />
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default()); 
    yew::Renderer::<App>::new().render();
}
