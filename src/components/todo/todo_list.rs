use yew::{function_component, html, Html};
use crate::components::todo::todo_item::TodoItem;
use crate::components::todo::types::Todo;

#[function_component(TodoList)]
pub fn todo_list() -> Html {
    let todo_items = vec![
        Todo {
            id: 1,
            title: String::from("TodoA"),
            completed: true,
        },
        Todo {
            id: 2,
            title: String::from("TodoB"),
            completed: true,
        },
        Todo {
            id: 3,
            title: String::from("TodoC"),
            completed: false,
        },
    ];

    html! {
        <ul class="list-group">
            {todo_items.iter().map(|todo| html! {
                <TodoItem title={todo.title.clone()} completed={todo.completed} />
            }).collect::<Html>()}
        </ul>
    }
}
