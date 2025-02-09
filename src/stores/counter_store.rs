use yewdux::prelude::*;
use yewdux::Context;

#[derive(Default, PartialEq, Clone, Debug)]
pub struct CounterState {
    pub count: i32,
}

impl Store for CounterState {
    fn new(_cx: &Context) -> Self {
        Self { count: 0 }
    }

    fn should_notify(&self, _old: &Self) -> bool {
        self != _old
    }
}
