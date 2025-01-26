use std::str::FromStr;
use stylist::ast::Sheet;
use stylist::yew::styled_component;
use stylist::Style;
use yew::prelude::*;
mod components;

use components::atoms::main_title::MainTitle;

const STYLE_FILE: &str = include_str!("styles/main.css");

#[styled_component]
pub fn App() -> Html {
    let sheet = Sheet::from_str(STYLE_FILE).unwrap();
    let stylesheets = Style::new(sheet).unwrap();

    let message = Some("I am a message");
    let tasks = vec!["Learn Rust", "Learn Yew", "Build cool stuff"];

    html! {
        <div class={stylesheets}>
            <MainTitle title="Hi there!" />
            <p>{"The class is my-class"}</p>

            if let Some(message) = message {
                <p>{message}</p>
            }

            <ul>
                { list_to_html(tasks) }
            </ul>
        </div>
    }
}

fn list_to_html(list: Vec<&str>) -> Vec<Html> {
    list.iter().map(|item| html! { <li>{item}</li> }).collect()
}
