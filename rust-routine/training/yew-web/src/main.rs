use yew::prelude::*;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div>
            <MainTitle title="Hello World"/>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: String,
}

#[function_component(MainTitle)]
pub fn main_title(props: &Props) -> Html {
    html! {
        <div>
            <h1>{&props.title}</h1>
        </div>
    }
}
