use yewdux::Context;
use yewdux::prelude::*;

#[derive(Default, Clone, PartialEq, Debug)]
pub struct AuthState {
    pub username: Option<String>,
    pub password: Option<String>,
    pub is_authenticated: bool,
}

impl Store for AuthState {
    fn new(_cx: &Context) -> Self {
        Self {
            username: None,
            password: None,
            is_authenticated: false,
        }
    }

    fn should_notify(&self, _old: &Self) -> bool {
        self != _old
    }
}
