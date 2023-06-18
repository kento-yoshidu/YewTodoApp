use yew::{function_component, html, Html, use_state, Callback, InputEvent, MouseEvent};
use crate::Properties;

#[derive(Properties, PartialEq)]
pub struct TodoFormProps {
    pub on_add: Callback<String>
}

#[function_component(TodoForm)]
pub fn todo_form(props: &TodoFormProps) -> Html {
    let title = use_state(|| String::from(""));

    let oninput = {
        let title = title.clone();

        Callback::from(move |e: InputEvent| {
            let value = e.data();

            match value {
                Some(value) => {
                    title.set((*title).clone() + &value)
                }
                _ => {
                    title.set(String::from(""))
                }
            }
        })
    };

    let onclick = {
        let title = title.clone();
        let on_add = props.on_add.clone();

        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            title.set("".to_string());
            on_add.emit((*title).clone());
        })
    };

    html! {
        <form>
            <div>
                <label for="title" class="form-label">{"タイトル"}</label>
                <input
                    type="text"
                    class="form-control"
                    id="title"
                    value={(*title).clone()}
                    {oninput}
                />
            </div>

            <button
                type="submit"
                class="btn btn-primary"
                onclick={onclick}
            >
                {"追加"}
            </button>
        </form>
    }
}
