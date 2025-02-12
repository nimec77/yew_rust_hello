use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize, Store)]
#[store(storage = "session")]
pub struct TokenState {
    pub token: Option<String>,
}
