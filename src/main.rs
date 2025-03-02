mod app;
mod phoenix;
mod sqlite;
mod tui;

use app::App;
use phoenix::event::PhoenixEvent;
use phoenix::Phoenix;
use std::env;
use tokio::sync::{mpsc, watch};
use tui::TUI;

const SOCKET_ENDPOINT: &str = if cfg!(debug_assertions) {
    "ws://localhost:4000/tui/websocket"
} else {
    "wss://phoenix.aayushsahu.com/tui/websocket"
};

#[tokio::main]
async fn main() {
    // TODO: enable this for dev mode only
    cli_log::init_cli_log!();

    let (socket_tx, socket_rx) = mpsc::channel::<PhoenixEvent>(100);
    let (screen_tx, screen_rx) = mpsc::channel::<PhoenixEvent>(100);
    let (signal_close_tx, mut signal_close_rx) = watch::channel(false);

    let meowui_secret = env::var("MEOWUI_SECRET").unwrap();
    let phoenix_endpoint = format!("{}?secret={}", SOCKET_ENDPOINT, meowui_secret);

    let mut phoenix = Phoenix::new(phoenix_endpoint.as_str(), socket_tx, screen_rx);
    let phoenix_handle = tokio::spawn(async move {
        phoenix.setup().await;

        tokio::select! {
            _ = phoenix.run() => {
                // TODO: add print statement for `dev` only
            },
            _ = signal_close_rx.changed() => {}
        }

        phoenix.disassemble().await;
    });

    let app = App::new(socket_rx, screen_tx);
    let mut tui = TUI::new();
    match tui.run(app) {
        Ok(()) => {}
        Err(e) => eprintln!("{e}"),
    }

    let _ = signal_close_tx.send(true);
    let _ = tokio::join!(phoenix_handle);
}
