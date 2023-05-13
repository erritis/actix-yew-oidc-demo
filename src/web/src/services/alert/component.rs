use yew::{function_component, html, Callback, Html, MouseEvent, Properties};
use yew_notifications::Notifiable;

use super::model::{Alert, AlertType};

use yew_bootstrap::{
    component::Alert as AlertBootstrap,
    util::Color
};


impl From<&AlertType> for Color {
    fn from(alert_type: &AlertType) -> Self {
        match alert_type {
            AlertType::Info => Color::Primary,
            AlertType::Warn => Color::Warning,
            AlertType::Error => Color::Danger,
            AlertType::Success => Color::Success,
        }
    }
}


/// Props for [`AlertComponent`]
#[derive(Properties, Clone, PartialEq)]
pub struct AlertComponentProps {
    /// Alert object to render.
    pub alert: Alert,

    /// *onclick* event callback.
    pub onclick: Callback<MouseEvent>,

    /// *onenter* event callback.
    pub onenter: Callback<MouseEvent>,

    /// *onleave* event callback.
    pub onleave: Callback<MouseEvent>,
}

/// Bootstrap alert component.
#[function_component(AlertComponent)]
pub fn alert_component(props: &AlertComponentProps) -> Html {
    let text = &props.alert.text;
    let alert_type = &props.alert.alert_type;

    let style: Color = alert_type.into();

    let onclick = props.onclick.clone();
    let onenter = props.onenter.clone();
    let onleave = props.onleave.clone();

    let is_paused = if props.alert.is_paused() { "paused" } else { "" };

    html! {
        <div {onclick} onmouseenter={onenter} onmouseleave={onleave} style="display: flex;">
            <AlertBootstrap class={is_paused} style={style} text={text.clone()} />
        </div>
    }
}