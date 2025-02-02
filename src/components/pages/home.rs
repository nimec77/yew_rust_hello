use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::Route;

#[function_component]
pub fn Home() -> Html {
    html! {
        <div>
            <h1>{"Home"}</h1>
            <div>
                <Link<Route> to={Route::Hello}> {"To Hello"} </Link<Route>>
            </div>
        </div>
    }
}
