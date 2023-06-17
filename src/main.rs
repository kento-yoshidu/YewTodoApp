use yew::prelude::*;
use components::header::Header;
use components::todo::todo_list::TodoList;

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

    html! {
        <>
            <Global css={global_style} />

            <Header />

            <main>
                <h1>{ "Hello World" }</h1>

                <TodoList />
            </main>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
