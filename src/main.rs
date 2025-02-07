mod app;
mod phoenix;
mod tui;

use app::App;
use phoenix::Phoenix;
use tui::TUI;

#[tokio::main]
async fn main() {
    let mut phoenix = Phoenix::new("ws://localhost:4000/socket/websocket").await;

    // make the connection happen async so that we don't block on waiting for the connection to happen
    phoenix.connect().await;
    phoenix.join_phoenix_channel().await;

    let app = App::new(&phoenix);

    let mut tui = TUI::new();
    tui.run(app).unwrap();

    phoenix.disconnect_channel().await;
    phoenix.disconnect().await;
}
