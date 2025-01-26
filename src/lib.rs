use stylist::yew::styled_component;
use yew::prelude::*;
mod components;

use components::atoms::main_title::{MainTitle, Color};

#[styled_component]
pub fn App() -> Html {
    html! {
        <div>
            <MainTitle title="Hi there!" color={Color::Ok} />
        </div>
    }
}
