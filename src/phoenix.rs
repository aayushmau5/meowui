use phoenix_channels_client::Socket;
use std::{sync::Arc, time::Duration};
use url::Url;

pub struct Phoenix {
    pub socket: Arc<Socket>,
}

impl Phoenix {
    pub async fn new(url: &str) -> Self {
        let url = Url::parse(url).unwrap();
        let socket = Socket::spawn(url, None).await.unwrap();
        Self { socket }
    }

    pub async fn connect(&self) {
        self.socket.connect(Duration::from_secs(10)).await.unwrap();
    }

    pub async fn disconnect(&self) {
        self.socket.disconnect().await.unwrap();
    }
}
