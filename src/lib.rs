mod components;
mod router;

use components::molecules::struct_counter::StructCounter;
use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct User {
    pub username: String,
    pub favorite_language: String,
}

#[styled_component]
pub fn App() -> Html {
    html! {
        <StructCounter />
    }
}
