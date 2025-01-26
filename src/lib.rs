use gloo::console::log;
use stylist::yew::styled_component;
use yew::prelude::*;
mod components;

use components::atoms::main_title::{Color, MainTitle};
use crate::components::molecules::custom_form::CustomForm;

#[styled_component]
pub fn App() -> Html {
    let main_title_load = Callback::from(|message: String| log!(message));
    html! {
        <div>
            <MainTitle
                title="Hi there!"
                color={Color::Ok}
                on_load={main_title_load}
            />
            <CustomForm />
        </div>
    }
}
