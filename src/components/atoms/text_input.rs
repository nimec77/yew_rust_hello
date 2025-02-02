use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
    pub placeholder: String,
    pub handle_onchange: Callback<String>,
}
#[function_component(TextInput)]
pub fn text_input(props: &Props) -> Html {
    let onchange_callback = props.handle_onchange.clone();
    let onchange = Callback::from(move |event: Event| {
        let value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();
        onchange_callback.emit(value);
    });
    html! {
        <input
            type="text" name={props.name.clone()}
            placeholder={props.placeholder.clone()}
            onchange={onchange}
        />
    }
}
