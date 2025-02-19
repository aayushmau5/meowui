use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    prelude::*,
    widgets::{Block, Widget},
};

#[derive(Default, Clone)]
pub struct InputWidget<'a> {
    content: String,
    cursor_position: usize,
    input_style: Style,
    cursor_style: Style,
    block: Option<Block<'a>>,
}

impl<'a> InputWidget<'a> {
    pub fn new(content: String, input_style: Style, cursor_style: Style) -> Self {
        let content_len = content.len();
        Self {
            content,
            cursor_position: content_len,
            block: None,
            input_style,
            cursor_style,
        }
    }

    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn insert_char(&mut self, c: char) {
        self.content.insert(self.cursor_position, c);
        self.cursor_position += 1;
    }

    pub fn delete_char(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
            self.content.remove(self.cursor_position);
        }
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.cursor_position < self.content.len() {
            self.cursor_position += 1;
        }
    }

    pub fn handle_key(&mut self, e: KeyEvent) {
        match e.code {
            KeyCode::Left => self.move_cursor_left(),
            KeyCode::Right => self.move_cursor_right(),
            KeyCode::Char(c) => self.insert_char(c),
            KeyCode::Backspace => self.delete_char(),
            _ => {}
        }
    }
}

impl Widget for &InputWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Get the area to render the content
        let area = match &self.block {
            Some(b) => {
                b.render(area, buf);
                b.inner(area)
            }
            None => area,
        };

        // Calculate cursor position
        let cursor_x = self.cursor_position as u16;

        // Render the text content
        buf.set_string(area.x, area.y, &self.content, self.input_style);

        // Render the cursor
        if cursor_x < area.width {
            buf.set_style(
                Rect::new(area.x + cursor_x, area.y, 1, 1),
                self.cursor_style,
            );
        }
    }
}
