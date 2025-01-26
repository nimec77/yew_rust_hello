use crate::components::atoms::text_input::TextInput;
use crate::components::molecules::custom_button::CustomButton;
use yew::prelude::*;

#[function_component(CustomForm)]
pub fn custom_form() -> Html {
    let username_state = use_state(|| "no username set".to_owned());
    let cloned_username_state = username_state.clone();
    let username_change = Callback::from(move |username: String| {
        cloned_username_state.set(username);
    });
    html! {
        <form>
            <TextInput name="username" handle_onchange={username_change} />
            <CustomButton label="Submit" />
            <p>{"Username: "}{&*username_state}</p>
        </form>
    }
}
