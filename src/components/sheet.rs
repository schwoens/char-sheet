use std::ops::Deref;

use crate::components::text_input::TextInput;
use yew::prelude::*;

use crate::Character;

#[derive(Properties, PartialEq)]
pub struct SheetProps {}

#[function_component]
pub fn Sheet(_props: &SheetProps) -> Html {
    let state = use_state(Character::default);
    let cloned_state = state.clone();

    let name_onchange = Callback::from(move |name: String| {
        let mut char = cloned_state.deref().clone();
        char.name = name;
        cloned_state.set(char);
    });

    html! {
        <div class={classes!("flex", "flex-column")}>
            <TextInput
                name={"name"}
                value={state.name.clone()}
                onchange={name_onchange}
            />
        </div>
    }
}
