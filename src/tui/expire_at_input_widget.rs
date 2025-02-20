use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    prelude::*,
    widgets::{Block, Widget},
};

#[derive(Default, Clone)]
pub struct ExpireAtWidget<'a> {
    time: String,
    unit: TimeUnit,
    cursor_position: usize,
    input_style: Style,
    cursor_style: Style,
    selected_style: Style,
    unselected_style: Style,
    block: Option<Block<'a>>,
}

pub struct ExpireAt {
    pub time: usize,
    pub unit: TimeUnit,
}

#[derive(Clone, PartialEq)]
pub enum TimeUnit {
    Day,
    Minute,
    Hour,
}

impl Default for TimeUnit {
    fn default() -> Self {
        Self::Minute
    }
}

impl TimeUnit {
    fn next(&self) -> Self {
        match self {
            TimeUnit::Day => TimeUnit::Minute,
            TimeUnit::Minute => TimeUnit::Hour,
            TimeUnit::Hour => TimeUnit::Day,
        }
    }

    fn prev(&self) -> Self {
        match self {
            TimeUnit::Day => TimeUnit::Hour,
            TimeUnit::Minute => TimeUnit::Day,
            TimeUnit::Hour => TimeUnit::Minute,
        }
    }

    pub fn to_string(&self) -> &str {
        match self {
            TimeUnit::Day => "Day",
            TimeUnit::Minute => "Minute",
            TimeUnit::Hour => "Hour",
        }
    }
}

impl<'a> ExpireAtWidget<'a> {
    pub fn new(input_style: Style, cursor_style: Style) -> Self {
        Self {
            time: String::from("0"),
            unit: TimeUnit::Minute,
            cursor_position: 1,
            block: None,
            input_style,
            cursor_style,
            selected_style: Style::default().reversed(),
            unselected_style: Style::default(),
        }
    }

    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    pub fn time(&self) -> ExpireAt {
        let int_time: usize = self.time.parse().unwrap();
        ExpireAt {
            time: int_time,
            unit: self.unit.clone(),
        }
    }

    pub fn insert_char(&mut self, c: char) {
        self.time.insert(self.cursor_position, c);
        self.cursor_position += 1;
    }

    pub fn delete_char(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
            self.time.remove(self.cursor_position);
        }
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.cursor_position < self.time.len() {
            self.cursor_position += 1;
        }
    }

    pub fn handle_key(&mut self, e: KeyEvent) {
        match e.code {
            KeyCode::Left => self.move_cursor_left(),
            KeyCode::Right => self.move_cursor_right(),
            KeyCode::Char('l') => self.unit = self.unit.next(),
            KeyCode::Char('h') => self.unit = self.unit.prev(),
            KeyCode::Char(c) => {
                if c.is_ascii_digit() {
                    self.insert_char(c);
                }
            }
            KeyCode::Backspace => self.delete_char(),
            _ => {}
        }
    }
}

impl Widget for &ExpireAtWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Get the area to render the content
        let area = match &self.block {
            Some(b) => {
                b.render(area, buf);
                b.inner(area)
            }
            None => area,
        };

        // Calculate positions
        let cursor_x = self.cursor_position as u16;

        // Render the numeric input
        buf.set_string(area.x, area.y, &self.time, self.input_style);

        // Render the cursor for numeric input
        if cursor_x < self.time.len() as u16 + 1 {
            buf.set_style(
                Rect::new(area.x + cursor_x, area.y, 1, 1),
                self.cursor_style,
            );
        }

        // Render the time units
        let units_x = area.x + self.time.len() as u16 + 2;
        let units = ["Day", "Minute", "Hour"];
        let mut current_x = units_x;

        for unit in units.iter() {
            let style = if *unit == self.unit.to_string() {
                self.selected_style
            } else {
                self.unselected_style
            };

            buf.set_string(current_x, area.y, *unit, style);
            current_x += unit.len() as u16 + 1;
        }
    }
}
