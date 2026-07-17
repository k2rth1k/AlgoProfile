use crate::ui::components::counter_app::counter_app::CounterApp;
use crate::ui::components::listed_search::listed_search::ListedSearch;
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::{DefaultTerminal, Frame};
use std::io;

#[derive(Debug)]
pub struct ComponentState {
    pub focused: bool,
    pub visible: bool,
}

impl ComponentState {
    pub fn new() -> Self {
        Self {
            focused: false,
            visible: true,
        }
    }
}

pub trait FocusedComponent: std::fmt::Debug {
    fn set_focus(&mut self, focus: bool);
    fn is_focused(&self) -> bool;
    fn handle_key_events(&mut self, event: KeyEvent) -> Result<(), Box<dyn std::error::Error>>;
    fn render(&self, area: Rect, buf: &mut Buffer);
    fn cursor_position(&self, _area: Rect) -> Option<(u16, u16)> {
        None
    }
}

#[derive(Debug)]
pub struct App {
    focused_components: Vec<Box<dyn FocusedComponent>>,
    close: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            focused_components: vec![Box::new(CounterApp::new()), Box::new(ListedSearch::new())],
            close: false,
        }
    }
}
impl App {
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        // Show cursor
        crossterm::execute!(std::io::stdout(), crossterm::cursor::Show)?;

        while !self.close {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(frame.area());

        let mut cursor_pos = None;

        for (idx, component) in self.focused_components.iter().enumerate() {
            if idx < chunks.len() {
                component.render(chunks[idx], frame.buffer_mut());

                // Get cursor position from component if available
                if let Some(pos) = component.cursor_position(chunks[idx]) {
                    if component.is_focused() {
                        cursor_pos = Some(pos);
                    }
                }
            }
        }

        // Set cursor position if any component returned one
        if let Some((x, y)) = cursor_pos {
            frame.set_cursor_position((x, y));
        }
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        // Check for Ctrl+C
        if key_event.code == KeyCode::Char('c')
            && key_event.modifiers.contains(KeyModifiers::CONTROL)
        {
            self.close = true;
            return;
        }

        if key_event.code == KeyCode::Tab {
            self.cycle_focus();
        }

        match key_event.code {
            _ => {
                let focused_component = self.focused_components.iter_mut().find(|c| c.is_focused());

                if let Some(focusedComponent) = focused_component {
                    let _ = focusedComponent.handle_key_events(key_event);
                } else {
                    self.cycle_focus();
                    let _ = self.focused_components[0].handle_key_events(key_event);
                }
            }
        }
    }

    fn cycle_focus(&mut self) {
        if let Some(current_idx) = self.focused_components.iter().position(|c| c.is_focused()) {
            // Unfocus current component
            self.focused_components[current_idx].set_focus(false);
            let next_idx = (current_idx + 1) % self.focused_components.len();
            self.focused_components[next_idx].set_focus(true);
        } else {
            if !self.focused_components.is_empty() {
                self.focused_components[0].set_focus(true);
            }
        }
    }
}
