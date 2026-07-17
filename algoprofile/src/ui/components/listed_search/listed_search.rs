use crate::ui::main_ui::{ComponentState, FocusedComponent};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Line, Style, Stylize, Text, Widget};
use ratatui::style::Color;
use ratatui::symbols::border;
use ratatui::widgets::{Block, Paragraph};

#[derive(Debug)]
pub struct ListedSearch {
    state: ComponentState,
    list: Vec<String>,
    search_text: String,
    search_text_index: usize,
}

impl ListedSearch {
    pub fn new() -> ListedSearch {
        ListedSearch {
            state: ComponentState::new(),
            list: vec![],
            search_text: "".to_string(),
            search_text_index: 0,
        }
    }
    pub fn new_with_list(list: Vec<String>) -> ListedSearch {
        ListedSearch {
            list,
            search_text: "".to_string(),
            search_text_index: 0,
            state: ComponentState::new(),
        }
    }

    fn move_cursor_left(&mut self) {
        let cursor = self.search_text_index.saturating_sub(1);
        self.search_text_index = self.clamp_cursor(cursor);
    }

    fn move_cursor_right(&mut self) {
        let cursor = self.search_text_index.saturating_add(1);
        self.search_text_index = self.clamp_cursor(cursor);
    }

    fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.search_text.insert(index, new_char);
        self.move_cursor_right();
    }

    fn clamp_cursor(&mut self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.search_text.chars().count())
    }

    fn byte_index(&self) -> usize {
        self.search_text
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.search_text_index)
            .unwrap_or(self.search_text.len())
    }

    fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.search_text_index != 0;
        if is_not_cursor_leftmost {
            let current_index = self.search_text_index;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.search_text.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.search_text.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.search_text = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    fn reset_cursor(&mut self) {
        self.search_text_index = 0;
    }
}

impl FocusedComponent for ListedSearch {
    fn set_focus(&mut self, focus: bool) {
        self.state.focused = focus;
    }

    fn is_focused(&self) -> bool {
        self.state.focused
    }

    fn handle_key_events(&mut self, event: KeyEvent) -> Result<(), Box<dyn std::error::Error>> {
        match event.code {
            KeyCode::Char(to_insert) => self.enter_char(to_insert),
            KeyCode::Backspace => self.delete_char(),
            KeyCode::Left => self.move_cursor_left(),
            KeyCode::Right => self.move_cursor_right(),
            _ => {}
        }
        Ok(())
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        Widget::render(self, area, buf);
    }

    fn cursor_position(&self, area: Rect) -> Option<(u16, u16)> {
        Some((area.x + self.search_text_index as u16 + 1, area.y + 1))
    }
}

impl Widget for &ListedSearch {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let right_block = Block::bordered()
            .title(Line::from(" Text Input ".bold()).centered())
            .style(Style::default().fg(if self.is_focused() {
                Color::Blue
            } else {
                Color::White
            }))
            .border_set(border::THICK);

        let input_text = Text::from(self.search_text.as_str());

        Paragraph::new(input_text)
            .block(right_block)
            .render(area, buf);
    }
}
