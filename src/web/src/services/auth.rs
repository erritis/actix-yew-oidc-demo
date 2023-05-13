
use time::Duration;
use yew::prelude::*;
use yew_oauth2::prelude::*;
use yew_oauth2::openid::*;
use yew_notifications::use_notification;
use gloo_storage::{SessionStorage, Storage};


use crate::{opts::Opts, services::{config::use_config, alert::model::{Alert, AlertType}}};



/// Props for [`AuthProvider`]
#[derive(Properties, Clone, PartialEq)]
pub struct AuthProviderProps {
    pub children: Children,
}
#[function_component(AuthProvider)]
pub fn auth_provider(
    props: &AuthProviderProps,
) -> Html {

    let children = props.children.clone();

    let opts: Opts = use_config();

    let config = Config {
        client_id: opts.client_id.clone(),
        issuer_url: opts.issuer_url.clone(),
        additional: Additional {
            after_logout_url: Some("/".into()),
            ..Default::default()
        },
    };

    html! {
        <OAuth2 {config} scopes={vec!["openid".to_string()]}>
            <AuthStateProvider>
                {children}
            </AuthStateProvider>
        </OAuth2>
    }
}


/// Props for [`AuthStateProvider`]
#[derive(Properties, Clone, PartialEq)]
pub struct AuthStateProviderProps {
    pub children: Children,
}
#[function_component(AuthStateProvider)]
pub fn auth_state_provider(
    props: &AuthStateProviderProps,
) -> Html {

    let children = props.children.clone();

    let auth = use_auth_state();

    let agent = use_auth_agent().expect("Requires OAuth2Context component in parent hierarchy (Agent<OpenIdClient>)");

    let alert_manager = use_notification::<Alert>();


    use_effect_with_deps(|(auth, alert_manager)|
        {
            if let Some(OAuth2Context::Failed(message)) = auth {
                alert_manager.spawn(Alert::new(
                    AlertType::Error,
                    message.clone(),
                    Duration::seconds(30),
                ));
            }
        },(auth, alert_manager)
    );

    use_effect_with_deps(|agent|
    {
        if let Ok(true) = SessionStorage::get("is_authenticated") {
            let _ = agent.start_login();
        }
    }, agent);

    html! {
        <>
            {children}
        </>
    }
}
