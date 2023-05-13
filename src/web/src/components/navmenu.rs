mod navbar;
mod nav_container;

use yew::prelude::*;
use yew_oauth2::prelude::*;
use yew_oauth2::openid::*;

use yew_bootstrap::{component::*, util::Color};

use gloo_storage::{SessionStorage, Storage};

use crate::components::navmenu::{
    nav_container::NavContainer,
    navbar::Navbar
};

fn get_username(auth: &OAuth2Context) -> String {
    match auth.claims() {
        Some(claims) => {
            match claims.preferred_username() {
                Some(preferred_username) => preferred_username.to_string(),
                None => "".to_string()
            }
        },
        None => "".to_string()
    }
}


#[function_component(NavMenu)]
pub fn navmenu() -> Html {
    let brand = BrandType::BrandIcon {
        text: AttrValue::from("Yew Client"),
        url: Some(AttrValue::from("/")),
        icon: AttrValue::from("rocket")
    };

    let auth: OAuth2Context = use_auth_state().expect("Requires OAuth2Context component in parent hierarchy");

    let agent = use_auth_agent().expect("Requires OAuth2Context component in parent hierarchy (Agent<OpenIdClient>)");

    let login = {
        let agent = agent.clone();
        Callback::from(move |_: MouseEvent| {
            if let Ok(()) = agent.start_login() {
                let _ = SessionStorage::set("is_authenticated", true);
            };            
        })
    };
    let logout = Callback::from(move |_: MouseEvent| {
        let _ = SessionStorage::set("is_authenticated", false);
        let _ = agent.logout();
    });

    html! {
        <header>
            <Navbar brand={brand}>
                <NavContainer>
                    <NavItem text="Home" url="/home" />
                    <NavItem text="Client" url="/client" />
                    <NavItem text="Server" url="/server" />
                    <Authenticated>
                        <h6>{format!("Hello, {}", get_username(&auth))}</h6>
                        <NavItem>
                            <Button
                            style={Color::Warning}
                            onclick={logout}
                            >{"Logout"}
                            </Button>
                        </NavItem>
                    </Authenticated>
                    <NotAuthenticated>
                        <Button
                        style={Color::Warning}
                        onclick={login}
                        >{"Login"}
                        </Button>
                    </NotAuthenticated>
                </NavContainer>
            </Navbar>
        </header>
    }
}