use std::ops::Deref;

use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TextInputProps {
    pub name: String,
    pub value: String,
    pub onchange: Callback<String>,
}

#[function_component]
pub fn TextInput(props: &TextInputProps) -> Html {
    let active = use_state(|| false);
    let value = use_state(|| props.value.clone());

    let handle_onchange = props.onchange.clone();
    let cloned_value = value.clone();
    let onchange = Callback::from(move |event: Event| {
        let value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();
        cloned_value.set(value.clone());
        handle_onchange.emit(value);
    });

    let cloned_active = active.clone();
    let span_onclick = Callback::from(move |_event: MouseEvent| {
        cloned_active.set(true);
    });

    let cloned_active = active.clone();
    let input_onblur = Callback::from(move |_event: FocusEvent| {
        cloned_active.set(false);
    });

    html! {
    <>
        <div class={classes!("flex", "gap-2")}>
            <label for={props.name.clone()}>{props.name.clone()}</label>
            if *active {
                <input id={props.name.clone()} type={"text"} name={props.name.clone()} value={value.deref().to_string()} onblur={input_onblur} onchange={onchange}/>
            } else {
                <span class={classes!("input")} onclick={span_onclick}>{value.deref()}</span>
            }
        </div>
    </>
    }
}
