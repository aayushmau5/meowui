use crate::app::ScreenType;
use phoenix_channels_client::{Payload, JSON};
use std::fmt::Display;

/// Generic event over phoenix socket used for both 'send' and 'receive'
pub struct PhoenixEvent {
    pub name: String,
    pub payload: Option<serde_json::Value>,
}

impl PhoenixEvent {
    pub fn for_screen(&self) -> ScreenType {
        match self.name.as_str() {
            "main" => ScreenType::Main,
            "bin" => ScreenType::Bin,
            "notes" => ScreenType::Notes,
            "projects" => ScreenType::Projects,
            "todos" => ScreenType::Todos,
            _ => panic!("Not implemented for event: {}", self.name.as_str()),
        }
    }
}

impl Display for PhoenixEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {:?}", self.name, self.payload)
    }
}

impl From<Payload> for PhoenixEvent {
    fn from(value: Payload) -> Self {
        match value {
            Payload::JSONPayload { json } => {
                let json_value: serde_json::Value = JSON::into(json);
                Self {
                    name: json_value["name"].as_str().unwrap().to_string(),
                    payload: json_value.get("payload").cloned(),
                }
            }
            _ => panic!("Expecting JSON, received Binary payload."),
        }
    }
}
