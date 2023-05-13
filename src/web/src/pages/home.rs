use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
      <div>
        <h1>{"Hello, world!"}</h1>
        <p>{"Welcome to your new single-page application"}</p>
      </div>
    }
}
