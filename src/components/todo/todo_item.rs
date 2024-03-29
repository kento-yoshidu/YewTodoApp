use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct TodoItemProps {
    pub title: String,
    pub completed: bool,
}

#[function_component(TodoItem)]
pub fn todo_item(props: &TodoItemProps) -> Html {
    html! {
        <li>
            <input type="checkbox" checked={props.completed} />

            {&props.title}
        </li>
    }
}
