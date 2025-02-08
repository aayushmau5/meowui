mod app;
mod phoenix;
mod tui;

use app::App;
use phoenix::Phoenix;
use tokio::sync::{broadcast, mpsc, watch};
use tui::TUI;

#[tokio::main]
async fn main() {
    let (socket_tx, socket_rx) = broadcast::channel::<String>(100);
    let (screen_tx, screen_rx) = mpsc::channel::<String>(100);
    let (signal_close_tx, mut signal_close_rx) = watch::channel(false);

    let mut phoenix = Phoenix::new("ws://localhost:4000/socket/websocket", socket_tx, screen_rx);

    let phoenix_handle = tokio::spawn(async move {
        phoenix.setup().await;

        tokio::select! {
            _ = phoenix.run() => {
                println!("PHX Done");
            },
            _ = signal_close_rx.changed() => {
                println!("Closing signal received")
            }
        }

        phoenix.disassemble().await;
    });

    let app = App::new();
    let mut tui = TUI::new();
    tui.run(app).unwrap();

    let _ = signal_close_tx.send(true);
    let _ = tokio::join!(phoenix_handle);
}
