use yew::prelude::*;

use yew_bootstrap::{component::*, util::Color};
use yew_oauth2::prelude::{OAuth2Context, use_auth_state, Reason};
use yew_router::prelude::use_route;
//use yew_bootstrap::util::*;
use crate::{components::navmenu::NavMenu, route::Route};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(Layout)]
pub fn layout(props: &Props) -> Html {

  let auth = use_auth_state();

  let route = use_route::<Route>();

  html! {
    <div>
      if route.is_some() && route != Some(Route::Home) && auth == Some(OAuth2Context::NotAuthenticated { reason: Reason::NewSession })
      {
        <div style="position: absolute; top: 50%; left: 50%;"><Spinner style={Color::Warning} /></div>
      }
      else {
        <NavMenu />
      }
      <Container>{ for props.children.iter() }</Container>
    </div>
  }
}