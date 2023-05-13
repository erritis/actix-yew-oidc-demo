use yew::prelude::*;
use stylist::yew::use_style;
use yew_bootstrap::component::{NavBar, BrandType};



#[derive(Properties, PartialEq)]
pub struct Props {
    pub brand: BrandType,
    pub children: Children,
}



#[function_component(Navbar)]
pub fn navbar(props: &Props) -> Html {
    let style = use_style!(
      r#"
      .navbar-nav {
        width: 100%;
      }
      "#
    );
    html! {
      <NavBar
            nav_id={"test-nav"}
            class={format!("navbar-expand-sm navbar-toggleable-sm ng-white border-bottom box-shadow mb-3 navbar-light bg-light flex-grow {}", style.get_class_name())}
            brand={props.brand.clone()}
            >
        { for props.children.iter() }
      </NavBar>
    }
}
