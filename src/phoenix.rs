use cli_log::info;
use phoenix_channels_client::{Channel, Event, Payload, Socket, Topic};
use serde_json::json;
use std::{sync::Arc, time::Duration};
use tokio::sync::broadcast::Sender as BroadcastSender;
use tokio::sync::mpsc::Receiver as ScreenReceiver;
use tokio::task::JoinHandle;
use url::Url;

pub struct Phoenix {
    pub url: Url,
    pub socket_tx: BroadcastSender<String>,
    pub screen_rx: ScreenReceiver<String>,
    pub socket: Option<Arc<Socket>>,
    pub channel: Option<Arc<Channel>>,
}

impl Phoenix {
    pub fn new(
        url: &str,
        socket_tx: BroadcastSender<String>,
        screen_rx: ScreenReceiver<String>,
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
                .channel(Topic::from_string("blog:events".to_string()), None)
                .await
                .unwrap();
            channel.join(Duration::from_secs(10)).await.unwrap();
            self.socket = Some(socket);
            self.channel = Some(channel);
        }
    }

    pub async fn disassemble(&mut self) {
        info!("Disassembled");
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
                        let payload = event.payload.to_string();
                        let _ = self.socket_tx.send(payload);
                    }
                }
            }
        }
    }

    pub async fn handle_screen_events(&mut self) {
        if let Some(value) = self.screen_rx.recv().await {
            if value == String::from("UPDATE_COUNT") {
                if let Some(channel) = &self.channel {
                    info!("Calling channel");
                    match channel
                        .call(
                            Event::from_string("like".to_string()),
                            Payload::json_from_serialized(
                                json!({"topic": "blog:events"}).to_string(),
                            )
                            .unwrap(),
                            Duration::from_secs(10),
                        )
                        .await
                    {
                        Ok(payload) => {
                            info!("received from phoenix: {payload}");
                            let _ = self.socket_tx.send(payload.to_string());
                        }
                        Err(e) => info!("error: {e}"),
                    }
                }
            }
            info!("received {value}");
        };
    }
}
