use yew::{function_component, html, Html};

mod time;
mod icons;
mod weather;

use time::Time;

#[function_component]
fn App() -> Html {
    html!(
        <div>
            <Time></Time>
        </div>
    )
}

fn main() {
    yew::Renderer::<App>::new().render();
}
