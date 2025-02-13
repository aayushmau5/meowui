use phoenix_channels_client::{Payload, JSON};
use std::fmt::Display;

/// Generic event over phoenix socket used for both 'send' and 'receive'
pub struct PhoenixEvent {
    pub name: String,
    pub payload: Option<serde_json::Value>,
}

impl PhoenixEvent {}

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
                    name: json_value.get("name").unwrap().to_string(),
                    payload: json_value.get("payload").cloned(),
                }
            }
            _ => panic!("Expecting JSON, received Binary payload."),
        }
    }
}
