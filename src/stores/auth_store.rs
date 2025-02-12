use yewdux::prelude::*;

#[derive(Default, Clone, PartialEq, Debug, Store)]
pub struct AuthState {
    pub username: Option<String>,
    pub password: Option<String>,
    pub is_authenticated: bool,
}
