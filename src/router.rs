use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::pages::{home::Home, hello::Hello};

#[derive(Routable, PartialEq, Clone)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/hello")]
    Hello,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::Hello => html! { <Hello /> },
    }
}
