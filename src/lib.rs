mod components;
mod router;
mod stores;
mod display_count;
mod increment_count;

use display_count::DisplayCount;
use increment_count::IncrementCount;
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
        <div>
            <h1>{ "Yewdux Counter" }</h1>
            <DisplayCount />
            <IncrementCount />
        </div>
    }
}
