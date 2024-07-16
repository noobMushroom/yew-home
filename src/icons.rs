use yew::{function_component, html, AttrValue, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Link {
    pub icon: AttrValue,
    pub link: AttrValue,
}

impl Link {
    pub fn new(icon: AttrValue, link: AttrValue) -> Self {
        Link { icon, link }
    }
}

#[derive(Properties, PartialEq)]
pub struct Links {
    pub link: Vec<Link>,
}

#[function_component]
fn Icon(link: &Link) -> Html {
    html!(
        <div>
            <p>{&link.icon}</p>
            <p>{&link.link}</p>
        </div>
    )
}

#[function_component]
fn RenderList(links: &Links) -> Html {
    html!({
        for links.link.iter().map(|link| {
            html!(
                    <Icon icon={&link.icon} link={&link.link}></Icon>
            )
        })
    })
}
