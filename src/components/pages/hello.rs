use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::Route;

#[function_component]
pub fn Hello() -> Html {
    let navigator = use_navigator().unwrap();
    let onclick: Callback<MouseEvent> = Callback::from(move |_| {
        let navigator = navigator.clone();
        navigator.push(&Route::Home);
    });

    html! {
        <div>
            <h1>{"Hello"}</h1>
            <button onclick={onclick}>{"Go Home"}</button>
        </div>
    }
}
