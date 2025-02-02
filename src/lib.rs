mod components;
mod router;

use gloo::console::log;
use std::ops::Deref;
use stylist::yew::styled_component;
use yew::ContextProvider;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::molecules::custom_form::{CustomForm, Data};
use crate::components::atoms::main_title::{Color, MainTitle};
use crate::router::{Route, switch};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct User {
    pub username: String,
    pub favorite_language: String,
}

#[styled_component]
pub fn App() -> Html {
    let user_state = use_state(User::default);
    let main_title_load = Callback::from(|message: String| log!(message));
    let first_load = use_state(|| true);

    use_effect(move || {
        if *first_load {
            first_load.set(false);
        }

        || {}
    });

    let custom_form_submit = {
        let user_state = user_state.clone();
        Callback::from(move |data: Data| {
            let mut user = user_state.deref().clone();
            user.username = data.username;
            user.favorite_language = data.favorite_language;
            user_state.set(user);
        })
    };

    html! {
        <ContextProvider<User> context={user_state.deref().clone()}>
            <MainTitle
                title="Hi there!"
                color={Color::Ok}
                on_load={main_title_load}
            />
            <CustomForm  onsubmit={custom_form_submit} />
            <BrowserRouter>
            <Switch<Route> render={switch} />
            </BrowserRouter>
        </ContextProvider<User>>
    }
}
