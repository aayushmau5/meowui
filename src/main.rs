mod app;
mod tui;

use app::App;
use tui::TUI;

fn main() {
    let mut tui = TUI::new();
    let app = App::new();
    tui.run(app).unwrap();
}
