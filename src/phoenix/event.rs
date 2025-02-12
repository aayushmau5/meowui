use phoenix_channels_client::{Payload, JSON};
use std::fmt::Display;

pub struct PhoenixEvent {
    pub from: String,
    pub payload: Option<serde_json::Value>,
}

impl PhoenixEvent {}

impl Display for PhoenixEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {:?}", self.from, self.payload)
    }
}

impl From<Payload> for PhoenixEvent {
    fn from(value: Payload) -> Self {
        match value {
            Payload::JSONPayload { json } => {
                let json_value: serde_json::Value = JSON::into(json);
                Self {
                    from: json_value.get("from").unwrap().to_string(),
                    payload: json_value.get("payload").cloned(),
                }
            }
            _ => panic!("Expecting JSON, received Binary payload."),
        }
    }
}
