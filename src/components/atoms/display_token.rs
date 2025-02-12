use yew::prelude::*;
use yewdux::prelude::*;

use crate::stores::token_store::TokenState;

#[function_component]
pub fn DisplayToken() -> Html {
    let (state, _) = use_store::<TokenState>();
    html! {
        <div>
            <h2>{"Token: "}{state.token.clone().unwrap_or_default()}</h2>
        </div>
    }
}
