use std::ops::Deref;
use crate::components::atoms::text_input::TextInput;
use crate::components::molecules::custom_button::CustomButton;
use yew::prelude::*;

#[derive(Clone, Default)]
struct Data {
    pub username: String,
    pub count: u32,
}

#[function_component(CustomForm)]
pub fn custom_form() -> Html {
    let state = use_state(|| Data::default());

    let cloned_state = state.clone();
    let username_change = Callback::from(move |username: String| {
        let mut data = cloned_state.deref().clone();
        data.username = username;
        cloned_state.set(data);
    });

    let cloned_state = state.clone();
    let button_clicked = Callback::from(move |_| {
        let mut data = cloned_state.deref().clone();
        data.count += 1;
        cloned_state.set(data);
    });

    html! {
        <div>
            <TextInput name="username" handle_onchange={username_change} />
            <CustomButton label="Submit" onclick={button_clicked} />
            <p>{"Username: "}{&state.username}</p>
            <p>{"Button clicked count: "}{state.count}</p>
        </div>
    }
}
