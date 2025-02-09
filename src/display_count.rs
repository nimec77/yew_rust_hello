use yew::prelude::*;
use yewdux::prelude::*;

use crate::stores::counter_store::CounterState;

#[function_component]
pub fn DisplayCount() -> Html {
    let (state, _) = use_store::<CounterState>();
    html! {
        <div>
            <h2>{ "Count" }</h2>
            <div>{ state.count }</div>
        </div>
    }
}
