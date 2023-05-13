
use yew::prelude::*;
use yew_notifications::{NotificationsProvider, NotificationsPosition};
use stylist::yew::use_style;
use super::{model::Alert, factory::AlertFactory};


/// Props for [`AlertProvider`]
#[derive(Properties, Clone, PartialEq)]
pub struct AlertProviderProps {
    pub children: Children,
}
#[function_component(AlertProvider)]
pub fn alert_provider(
    props: &AlertProviderProps,
) -> Html {

    let style = use_style!(
        r#"
        right: 2em;
        bottom: 2em;

        position: fixed;
        display: flex;
        flex-direction: column;
        gap: 0.5em;
        "#
    );
    let classes = classes!(style.get_class_name().to_owned());
    let position = use_state(|| NotificationsPosition::Custom(classes));
    let position_value = (*position).clone();
    let component_creator = AlertFactory::default();

    let children = props.children.clone();

    html! {
        <NotificationsProvider<Alert, AlertFactory> {component_creator} position={position_value}>
            {children}
        </NotificationsProvider<Alert, AlertFactory>>
    }
}
