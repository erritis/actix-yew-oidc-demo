use yew::{html, Callback, Html, MouseEvent};
use yew_notifications::NotifiableComponentFactory;

use super::{model::Alert, component::AlertComponent};

/// Standard alert factory.
///
/// This factory used for [`Alert`] components creation.
#[derive(Clone, PartialEq, Default)]
pub struct AlertFactory;

impl NotifiableComponentFactory<Alert> for AlertFactory {
    fn component(
        &self,
        alert: Alert,
        onclick: Callback<MouseEvent>,
        onenter: Callback<MouseEvent>,
        onleave: Callback<MouseEvent>,
    ) -> Html {
        html! {
            <AlertComponent {alert} {onclick} {onenter} {onleave} />
        }
    }
}