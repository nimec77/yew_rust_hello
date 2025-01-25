use gloo::console::log;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use stylist::ast::Sheet;
use stylist::yew::styled_component;
use stylist::Style;
use yew::{html, Html};

const STYLE_FILE: &str = include_str!("styles/main.css");

#[derive(Serialize, Deserialize)]
struct MyObject {
    username: String,
    favorite_language: String,
}
#[styled_component]
pub fn App() -> Html {
    let sheet = Sheet::from_str(STYLE_FILE).unwrap();
    let stylesheets = Style::new(sheet).unwrap();

    let name = "Dmitry";
    let my_object = MyObject {
        username: "Dmitry".to_owned(),
        favorite_language: "Rust".to_owned(),
    };

    log!("My name is ", name);
    log!(serde_json::to_string_pretty(&my_object).unwrap());

    let message = Some("I am a message");
    let tasks = vec!["Learn Rust", "Learn Yew", "Build cool stuff"];

    html! {
        <div class={stylesheets}>
            <h1>{"Hello, World!!!"}</h1>
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
