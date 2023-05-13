use yew::prelude::*;
use stylist::yew::use_style;



#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}



#[function_component(NavContainer)]
pub fn nav_container(props: &Props) -> Html {
    let style = use_style!(
        r#"
            display: flex;
            flex-direction: row;
            justify-content: end;
            align-items: center;
            width: 100%;
        "#
    );
    html! {
      <div class={style}>
        { for props.children.iter() }
      </div>
    }
}
