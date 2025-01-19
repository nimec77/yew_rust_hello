use gloo::console::log;
use serde::{Deserialize, Serialize};
use stylist::style;
use stylist::yew::styled_component;
use yew::{html, Html};

#[derive(Serialize, Deserialize)]
struct MyObject {
    username: String,
    favorite_language: String,
}
#[styled_component]
pub fn App() -> Html {
    let stylesheets = style!(
        r#"
            background-color: black;
            color: aqua;

            h1 {
                color: white;
            }
        "#
    ).unwrap();
    let name = "Dmitry";
    let my_object = MyObject {
        username: "Dmitry".to_owned(),
        favorite_language: "Rust".to_owned(),
    };

    log!("My name is ", name);
    log!(serde_json::to_string_pretty(&my_object).unwrap());

    let my_class = "my-class";
    let message = Some("I am a message");
    let tasks = vec!["Learn Rust", "Learn Yew", "Build cool stuff"];

    html! {
        <div class={stylesheets}>
            <h1 class={my_class}>{"Hello, World!!!"}</h1>
            if my_class == "my-class" {
                <p>{"The class is my-class"}</p>
            } else {
                <p>{"The class is not my-class"}</p>
            }

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
