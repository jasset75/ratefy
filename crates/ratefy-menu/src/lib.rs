use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
    Terminal,
};

use crossterm::event::{self, Event, KeyCode};
use std::time::Duration;

/// Generic menu item that leads to any `T` (typically a screen/state enum)
pub struct MenuItem<T> {
    pub label: String,
    pub key: u8,
    pub hint: Option<String>,
    pub next: T,
}

impl<T> MenuItem<T> {
    pub fn new(label: &str, key: u8, hint: Option<&str>, next: T) -> Self {
        MenuItem {
            label: label.to_string(),
            key,
            hint: hint.map(|h| h.to_string()),
            next,
        }
    }
}

/// Runs a TUI menu and returns the selected item's associated `next` value.
pub fn run_menu<T, B: Backend>(
    terminal: &mut Terminal<B>,
    title: &str,
    items: &[MenuItem<T>],
) -> Result<T, Box<dyn std::error::Error>>
where
    T: Copy,
{
    let mut selected = 0;

    loop {
        terminal.draw(|f| {
            let size = f.size();

            let layout = Layout::default()
                .direction(Direction::Vertical)
                .margin(5)
                .constraints([Constraint::Percentage(100)])
                .split(size);

            let list_items: Vec<ListItem> = items
                .iter()
                .enumerate()
                .map(|(_, item)| {
                    let mut line = format!("{}. {}", item.key, item.label);
                    if let Some(hint) = &item.hint {
                        line.push_str(&format!("  ({})", hint));
                    }
                    ListItem::new(line)
                })
                .collect();

            let mut state = ListState::default();
            state.select(Some(selected));

            let list = List::new(list_items)
                .block(Block::default().title(title).borders(Borders::ALL))
                .highlight_style(
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
                )
                .highlight_symbol("> ");

            f.render_stateful_widget(list, layout[0], &mut state);
        })?;

        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Up => {
                        selected = if selected == 0 {
                            items.len() - 1
                        } else {
                            selected - 1
                        };
                    }
                    KeyCode::Down => {
                        selected = (selected + 1) % items.len();
                    }
                    KeyCode::Enter => return Ok(items[selected].next),
                    KeyCode::Char(c) if c.is_ascii_digit() => {
                        if let Some(index) = items.iter().position(|i| i.key == c.to_digit(10).unwrap() as u8) {
                            return Ok(items[index].next);
                        }
                    }
                    KeyCode::Esc => return Err("User aborted with Esc".into()),
                    _ => {}
                }
            }
        }
    }
}
