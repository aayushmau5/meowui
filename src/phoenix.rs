pub mod event;

use cli_log::info;
use event::PhoenixEvent;
use phoenix_channels_client::{Channel, Event, Payload, Socket, Topic};
use serde_json::json;
use std::{sync::Arc, time::Duration};
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::task::JoinHandle;
use url::Url;

const CHANNEL_TOPIC: &str = "events";

pub struct Phoenix {
    pub url: Url,
    pub socket_tx: Sender<PhoenixEvent>,
    pub screen_rx: Receiver<PhoenixEvent>,
    pub socket: Option<Arc<Socket>>,
    pub channel: Option<Arc<Channel>>,
}

impl Phoenix {
    pub fn new(
        url: &str,
        socket_tx: Sender<PhoenixEvent>,
        screen_rx: Receiver<PhoenixEvent>,
    ) -> Self {
        let url = Url::parse(url).unwrap();

        Self {
            url,
            socket_tx,
            screen_rx,
            socket: None,
            channel: None,
        }
    }

    pub async fn setup(&mut self) {
        let socket = Some(Socket::spawn(self.url.clone(), None).await.unwrap());
        if let Some(socket) = socket {
            socket.connect(Duration::from_secs(10)).await.unwrap();
            let channel = socket
                .channel(Topic::from_string(CHANNEL_TOPIC.to_string()), None)
                .await
                .unwrap();
            channel.join(Duration::from_secs(10)).await.unwrap();
            self.socket = Some(socket);
            self.channel = Some(channel);
        }
    }

    pub async fn disassemble(&mut self) {
        if let Some(socket) = &self.socket {
            if let Some(channel) = &self.channel {
                channel.leave().await.unwrap();
                self.channel = None;
            }
            socket.disconnect().await.unwrap();
            self.socket = None;
        }
    }

    pub async fn run(&mut self) -> JoinHandle<()> {
        let channel = self.channel.as_ref().unwrap().clone();
        let events = channel.events();

        loop {
            tokio::select! {
                _ = self.handle_screen_events() => {},

                event = events.event() => {
                    if let Ok(event) = event {
                        let _ = self.socket_tx.send(event.payload.into());
                    }
                }
            }
        }
    }

    pub async fn handle_screen_events(&mut self) {
        if let Some(value) = self.screen_rx.recv().await {
            if let Some(channel) = &self.channel {
                let payload = if let Some(payload) = value.payload {
                    payload
                } else {
                    json!({})
                };

                match channel
                    .call(
                        Event::from_string(value.name),
                        Payload::json_from_serialized(payload.to_string()).unwrap(),
                        Duration::from_secs(10),
                    )
                    .await
                {
                    Ok(payload) => {
                        info!("received from phoenix: {payload}");
                        let _ = self.socket_tx.send(payload.into());
                    }
                    Err(e) => info!("error: {e}"),
                }
            }
        };
    }
}
