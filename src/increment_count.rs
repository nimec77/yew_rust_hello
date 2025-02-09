use std::rc::Rc;

use yew::prelude::*;
use yewdux::*;

use crate::stores::counter_store::CounterState;

enum Msg {
    Increment,
}

impl Reducer<CounterState> for Msg {
    fn apply(self, mut counter_state: Rc<CounterState>) -> Rc<CounterState> {
        let state = Rc::make_mut(&mut counter_state);
        match self {
            Msg::Increment => state.count += 1,
        };

        counter_state
    }
}
#[function_component]
pub fn IncrementCount() -> Html {
    let (_, dispatch) = use_store::<CounterState>();

    let onclick = dispatch.apply_callback(|_| Msg::Increment);

    html! {
        <button onclick={onclick}>{ "Increment" }</button>
    }
}
