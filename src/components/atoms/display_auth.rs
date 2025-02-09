use yew::prelude::*;
use yewdux::prelude::*;

use crate::stores::auth_store::AuthState;

#[function_component]
pub fn DisplayAuth() -> Html {
    let (state, _) = use_store::<AuthState>();
    html! {
        <div>
            <h2>{"Username: "}{state.username.clone().unwrap_or_default()}</h2>
            <h2>{"Password: "}{state.password.clone().unwrap_or_default()}</h2>
            <h2>{"Login status: "}{state.is_authenticated.to_string()}</h2>
        </div>
    }
}
