use crate::ui::main_ui::{ComponentState, FocusedComponent};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Color, Line, Style, Stylize, Text};
use ratatui::symbols::border;
use ratatui::widgets::{Block, Paragraph, Widget};

#[derive(Debug)]
pub struct CounterApp {
    counter: u8,
    state: ComponentState,
}

impl CounterApp {
    pub fn new() -> Self {
        CounterApp {
            counter: 0,
            state: ComponentState::new(),
        }
    }

    fn increment_counter(&mut self) {
        self.counter = self.counter.wrapping_add(1);
    }

    fn decrement_counter(&mut self) {
        self.counter = self.counter.wrapping_sub_signed(1);
    }
}

impl FocusedComponent for CounterApp {
    fn set_focus(&mut self, focus: bool) {
        self.state.focused = focus;
    }

    fn is_focused(&self) -> bool {
        self.state.focused
    }

    fn handle_key_events(&mut self, event: KeyEvent) -> Result<(), Box<dyn std::error::Error>> {
        match event.code {
            KeyCode::Left => self.decrement_counter(),
            KeyCode::Right => self.increment_counter(),
            _ => {}
        }
        Ok(())
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        Widget::render(self, area, buf);
    }
}

impl Widget for &CounterApp {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Counter App Tutorial ".bold());
        let instructions = Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .style(Style::default().fg(if self.is_focused() {
                Color::Blue
            } else {
                Color::White
            }))
            .border_set(border::THICK);

        let counter_text = Text::from(vec![Line::from(vec![
            "Value: ".into(),
            self.counter.to_string().yellow(),
        ])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
