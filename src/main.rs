use yew::prelude::*;
use components::header::Header;
use components::todo::todo_list::TodoList;
use components::todo::todo_form::TodoForm;
use components::todo::types::Todo;

use stylist::{css};
use stylist::yew::{Global};

mod components;

#[function_component(App)]
fn app() -> Html {
    let global_style = css!(r#"
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }
        html {
            font-family:
                -apple-system,
                BlinkMacSystemFont,
                Roboto,
                "Segoe UI",
                "Helvetica Neue",
                HelveticaNeue,
                YuGothic,
                "Yu Gothic Medium",
                "Yu Gothic",
                Verdana,
                Meiryo,
                sans-serif;
            scroll-behavior: smooth;
        }

        body {
            color: #444;
        }

        :root {
            --main-color: #399;
        }
    "#);

    let todo_items = use_state(|| Vec::<Todo>::new());
    let next_id = use_state(|| 1);

    let on_add = {
        let todo_items = todo_items.clone();
        Callback::from(move |title: String| {
            let mut current_todo_items = (*todo_items).clone();
            current_todo_items.push(Todo {
                id: *next_id,
                title,
                completed: false,
            });
            next_id.set(*next_id + 1);
            todo_items.set(current_todo_items);
        })
    };

    html! {
        <>
            <Global css={global_style} />

            <Header />

            <main>
                <h1>{ "Hello World" }</h1>

                <TodoForm {on_add} />
                <TodoList todo_items={(*todo_items).clone()} />
            </main>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
    wasm_logger::init(wasm_logger::Config::default());
}
