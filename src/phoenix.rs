use phoenix_channels_client::{Channel, Socket, SocketChannelError};
use std::{sync::Arc, time::Duration};
use url::Url;

pub struct Phoenix {
    pub socket: Arc<Socket>,
    pub phoenix_channel: Option<Arc<Channel>>,
}

impl Phoenix {
    pub async fn new(url: &str) -> Self {
        let url = Url::parse(url).unwrap();
        let socket = Socket::spawn(url, None).await.unwrap();
        Self {
            socket,
            phoenix_channel: None,
        }
    }

    pub async fn connect(&self) {
        self.socket.connect(Duration::from_secs(10)).await.unwrap();
    }

    pub async fn disconnect(&self) {
        self.socket.disconnect().await.unwrap();
    }

    pub async fn join_phoenix_channel(&mut self) {
        let channel_name = String::from("user-join");
        if let Ok(channel) = self.create_channel(channel_name).await {
            match channel.join(Duration::from_secs(10)).await {
                Ok(payload) => {
                    self.phoenix_channel = Some(channel);
                    println!("{payload:?}");
                }
                Err(_err) => eprintln!("Failed to join phoenix channel"),
            }
        } else {
            eprintln!("Failed to create phoenix channel");
        }
    }

    pub async fn disconnect_channel(&mut self) {
        if let Some(channel) = &self.phoenix_channel {
            let _ = channel.leave().await;
            self.phoenix_channel = None;
        }
    }

    async fn create_channel(
        &self,
        channel_name: String,
    ) -> Result<Arc<Channel>, SocketChannelError> {
        self.socket
            .channel(Arc::new(channel_name.into()), None)
            .await
    }
}
