mod components;
mod stores;

use components::{atoms::display_auth::DisplayAuth, molecules::auth_form::AuthForm};
use stylist::yew::styled_component;
use yew::prelude::*;

#[styled_component]
pub fn App() -> Html {
    html! {
        <div>
            <h1>{ "Yewdux Counter" }</h1>
            <AuthForm />
            <DisplayAuth />
        </div>
    }
}
