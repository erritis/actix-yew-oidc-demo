use yew::prelude::*;
use yew_router::prelude::*;
use yew_oauth2::openid::*;

use crate::pages::{home::Home, client::ClientPage, server::ServerPage};


#[derive(Debug, Clone, Routable, PartialEq, Eq)]
pub enum Route {
    #[at("/home")]
    Home,
    #[at("/client")]
    ClientPage,
    #[at("/server")]
    ServerPage,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        routes => {
            html! {
                <LocationRedirect logout_href="/home">
                {
                    match routes {
                        Route::ClientPage => html! { <ClientPage /> },
                        _ => html! { <ServerPage /> },
                    }
                }
                </LocationRedirect>
            }
        }
    }
}