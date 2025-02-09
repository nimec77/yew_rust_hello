use std::rc::Rc;

use gloo::console::log;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::stores::auth_store::AuthState;

pub enum Msg {
    UsernameInput(String),
    PasswordInput(String),
    Login,
}

impl Reducer<AuthState> for Msg {
    fn apply(self, mut auth_state: Rc<AuthState>) -> Rc<AuthState> {
        let state = Rc::make_mut(&mut auth_state);
        match self {
            Msg::Login => {
                state.is_authenticated = state.password.is_some() && state.username.is_some();
                log!("Login status: ", state.is_authenticated);
            },
            Msg::UsernameInput(username) => state.username = Some(username),
            Msg::PasswordInput(password) => state.password = Some(password),
        };
        auth_state
    }
}
#[function_component]
pub fn AuthForm() -> Html {
    let (_, dispatch) = use_store::<AuthState>();

    let onsubmit = dispatch.apply_callback(|event: SubmitEvent| {
        event.prevent_default();
        Msg::Login
    });


    let username_onchange = dispatch.apply_callback(|event: Event| {
        let target = event.target_unchecked_into::<HtmlInputElement>();
        let username = target.value();
        Msg::UsernameInput(username)
    });

    let password_onchange = dispatch.apply_callback(|event: Event| {
        let target = event.target_unchecked_into::<HtmlInputElement>();
        let password = target.value();
        Msg::PasswordInput(password)
    });

    html! {
        <form onsubmit={onsubmit}>
        <h2>{"Login"}</h2>
        <div>
            <label for="username">{"Username"}</label>
        </div>
            <input
                type="text"
                id="username"
                placeholder="Username"
                onchange={username_onchange}
            />
        <div>
            <label for="password">{"Password"}</label>
        </div>
            <input
                    type="password"
                    id="password"
                    placeholder="Password"
                    onchange={password_onchange}
            />
        <div>
            <button>
                {"Login"}
            </button>
        </div>
        </form>
    }
}
