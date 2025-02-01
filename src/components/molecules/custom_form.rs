use crate::components::atoms::text_input::TextInput;
use crate::components::molecules::custom_button::CustomButton;
use yew::prelude::*;

#[function_component(CustomForm)]
pub fn custom_form() -> Html {
    let username_state = use_state(|| "no username set".to_owned());
    let button_count_state = use_state(|| 0_u32);
    let cloned_username_state = username_state.clone();
    let username_change = Callback::from(move |username: String| {
        cloned_username_state.set(username);
    });
    let cloned_button_count_state = button_count_state.clone();
    let button_clicked = Callback::from(move |_| {
        let count = *cloned_button_count_state;
        cloned_button_count_state.set(count + 1);
    });
    html! {
        <div>
            <TextInput name="username" handle_onchange={username_change} />
            <CustomButton label="Submit" onclick={button_clicked} />
            <p>{"Username: "}{&*username_state}</p>
            <p>{"Button clicked count: "}{*button_count_state}</p>
        </div>
    }
}
