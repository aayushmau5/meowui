mod app;
mod phoenix;
mod tui;

use std::{thread, time::Duration};

use app::App;
use phoenix::Phoenix;
use tui::TUI;

#[tokio::main]
async fn main() {
    let phoenix = Phoenix::new("ws://localhost:4000/socket/websocket").await;
    phoenix.connect().await;
    thread::sleep(Duration::from_secs(10));
    phoenix.disconnect().await;
    let mut tui = TUI::new();
    let app = App::new();
    tui.run(app).unwrap();
}
