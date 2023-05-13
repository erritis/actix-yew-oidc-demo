use time::{Duration, OffsetDateTime};
use uuid::Uuid;
use yew_notifications::Notifiable;

/// Bootstrap alert type.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum AlertType {
    /// Represents some informative message for the user.
    #[default]
    Info,
    /// Represents some warning.
    Warn,
    /// Represents some error message.
    Error,
    /// Represents some success message.
    Success
}

impl From<&str> for AlertType {
    fn from(data: &str) -> Self {
        match data {
            "info" => Self::Info,
            "warn" => Self::Warn,
            "error" => Self::Error,
            "success" => Self::Success,
            _ => panic!("AlertType is not found!")
        }
    }
}

/// Bootstrap alert.
#[derive(Debug, Clone, PartialEq)]
pub struct Alert {
    pub(crate) id: Uuid,
    pub(crate) alert_type: AlertType,
    pub(crate) text: String,

    pub(crate) spawn_time: OffsetDateTime,
    pub(crate) lifetime: Duration,
    pub(crate) full_lifetime: Duration,
    pub(crate) paused: bool,
}

impl Alert {
    /// Creates a new bootstrap alert from alert type, title, text, and lifetime duration.
    pub fn new(
        alert_type: AlertType,
        text: impl Into<String>,
        lifetime: Duration,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            alert_type,
            text: text.into(),

            spawn_time: OffsetDateTime::now_local().expect("Can not acquire local current time"),
            lifetime,
            full_lifetime: lifetime,
            paused: false,
        }
    }
}

impl Notifiable for Alert {
    fn id(&self) -> Uuid {
        self.id
    }

    fn apply_tick(&mut self, time: Duration) {
        self.lifetime = self.lifetime.checked_sub(time).unwrap_or(Duration::default());
    }

    fn is_alive(&self) -> bool {
        self.lifetime != Duration::default()
    }

    fn mouse_in(&mut self) {
        self.paused = true;
    }

    fn mouse_out(&mut self) {
        self.paused = false;
        self.lifetime = self.full_lifetime;
    }

    fn is_paused(&self) -> bool {
        self.paused
    }
}