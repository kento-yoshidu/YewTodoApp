use yew::{function_component, html, Html};

use stylist::{style};

#[function_component(Header)]
pub fn header() -> Html {
    let nav_style = style!(r#"
        background-color: #444;
    "#).unwrap();

    let title_style = style!(r#"
        color: var(--main-color);
    "#).unwrap();

    html! {
        <nav class={nav_style}>
            <div>
                <h1
                    class={title_style}
                >
                    {"Yew Todo App"}
                </h1>
            </div>
        </nav>
    }
}
