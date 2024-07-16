use chrono::{DateTime, Datelike, Local, Timelike};
use yew::{function_component, html, Html};

#[function_component]
pub fn Time() -> Html {
    let time: DateTime<Local> = Local::now();
    let day = time.weekday().to_string();
    html!(
        <>
            <div>
                <p>{day}</p>
                <p>{time.hour()}{":"}{time.minute()}</p>
                <p>{"greeting"}</p>
            </div>
        </>
    )
}
