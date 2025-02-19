use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    prelude::*,
    widgets::{Block, Widget},
};

#[derive(Default, Clone)]
pub struct MultilineInput<'a> {
    lines: Vec<String>,
    cursor_x: usize,
    cursor_y: usize,
    block: Option<Block<'a>>,
}

impl<'a> MultilineInput<'a> {
    pub fn new(content: String) -> Self {
        let lines = if content.is_empty() {
            vec![content]
        } else {
            content.lines().map(String::from).collect()
        };
        let cursor_x = if lines.len() == 0 {
            0
        } else {
            lines.last().unwrap().len()
        };
        let cursor_y = if lines.len() == 0 { 0 } else { lines.len() - 1 };
        Self {
            lines,
            cursor_x,
            cursor_y,
            block: None,
        }
    }

    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    pub fn content(&self) -> String {
        self.lines.join("\n")
    }

    fn insert_char(&mut self, c: char) {
        if c == '\n' {
            // Handle new line
            let current_line = &self.lines[self.cursor_y];
            let remainder = current_line[self.cursor_x..].to_string();
            self.lines[self.cursor_y] = current_line[..self.cursor_x].to_string();
            self.lines.insert(self.cursor_y + 1, remainder);
            self.cursor_y += 1;
            self.cursor_x = 0;
        } else {
            // Insert character at cursor position
            self.lines[self.cursor_y].insert(self.cursor_x, c);
            self.cursor_x += 1;
        }
    }

    fn delete_char(&mut self) {
        if self.cursor_x > 0 {
            // Delete character within current line
            self.lines[self.cursor_y].remove(self.cursor_x - 1);
            self.cursor_x -= 1;
        } else if self.cursor_y > 0 {
            // Merge with previous line
            let current_line = self.lines.remove(self.cursor_y);
            self.cursor_y -= 1;
            self.cursor_x = self.lines[self.cursor_y].len();
            self.lines[self.cursor_y].push_str(&current_line);
        }
    }

    fn move_cursor_left(&mut self) {
        if self.cursor_x > 0 {
            self.cursor_x -= 1;
        } else if self.cursor_y > 0 {
            self.cursor_y -= 1;
            self.cursor_x = self.lines[self.cursor_y].len();
        }
    }

    fn move_cursor_right(&mut self) {
        if self.cursor_x < self.lines[self.cursor_y].len() {
            self.cursor_x += 1;
        } else if self.cursor_y < self.lines.len() - 1 {
            self.cursor_y += 1;
            self.cursor_x = 0;
        }
    }

    fn move_cursor_up(&mut self) {
        if self.cursor_y > 0 {
            self.cursor_y -= 1;
            self.cursor_x = self.cursor_x.min(self.lines[self.cursor_y].len());
        }
    }

    fn move_cursor_down(&mut self) {
        if self.cursor_y < self.lines.len() - 1 {
            self.cursor_y += 1;
            self.cursor_x = self.cursor_x.min(self.lines[self.cursor_y].len());
        }
    }

    pub fn handle_key(&mut self, e: KeyEvent) {
        match e.code {
            KeyCode::Up => self.move_cursor_up(),
            KeyCode::Down => self.move_cursor_down(),
            KeyCode::Left => self.move_cursor_left(),
            KeyCode::Right => self.move_cursor_right(),
            KeyCode::Enter => self.insert_char('\n'),
            KeyCode::Char(c) => self.insert_char(c),
            KeyCode::Backspace => self.delete_char(),
            _ => {}
        }
    }
}

impl Widget for &MultilineInput<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Get the area to render the content
        let inner_area = match &self.block {
            Some(b) => {
                b.render(area, buf);
                b.inner(area)
            }
            None => area,
        };

        // Render visible lines
        let visible_lines = inner_area.height as usize;
        for (i, line) in self.lines.iter().take(visible_lines).enumerate() {
            let y = inner_area.y + i as u16;
            if y < inner_area.y + inner_area.height {
                buf.set_string(inner_area.x, y, line, Style::default().fg(Color::White));
            }
        }

        let cursor_y: u16 = inner_area.y + self.cursor_y as u16;
        if self.cursor_x as u16 <= inner_area.width {
            buf.set_style(
                Rect::new(inner_area.x + self.cursor_x as u16, cursor_y, 1, 1),
                Style::default().fg(Color::Black).bg(Color::White),
            );
        }
    }
}
