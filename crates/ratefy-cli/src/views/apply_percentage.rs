use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::Text,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};
use ratefy_lib::{G10_CURRENCIES, apply_percentage_str};
use rust_decimal::Decimal;
use std::io;

/// Handles the percentage calculation screen
pub fn apply_percentage_view(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut input_base = String::new();
    let mut input_rate = String::new();
    let mut selected_currency_idx = 0;
    let mut step = 0;
    let mut result: Option<(Decimal, String)> = None;

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(5)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(1),
                    Constraint::Length(3),
                ])
                .split(size);

            let widget = match step {
                0 => Paragraph::new(Text::from(input_base.as_str()))
                    .block(
                        Block::default()
                            .title("Enter base value")
                            .borders(Borders::ALL),
                    )
                    .style(Style::default().add_modifier(Modifier::BOLD)),
                1 => Paragraph::new(Text::from(input_rate.as_str()))
                    .block(
                        Block::default()
                            .title("Enter percentage rate")
                            .borders(Borders::ALL),
                    )
                    .style(Style::default().add_modifier(Modifier::BOLD)),
                2 => {
                    let items: Vec<ListItem> = G10_CURRENCIES
                        .iter()
                        .map(|c| ListItem::new(c.to_string()))
                        .collect();

                    let mut state = ListState::default();
                    state.select(Some(selected_currency_idx));

                    let list = List::new(items)
                        .block(
                            Block::default()
                                .title("Select currency")
                                .borders(Borders::ALL),
                        )
                        .highlight_style(Style::default().add_modifier(Modifier::REVERSED));

                    f.render_stateful_widget(list, chunks[2], &mut state);

                    Paragraph::new("")
                }
                3 => {
                    let msg = match &result {
                        Some((amount, currency)) => {
                            format!("Result: {:.2} {}", amount, currency)
                        }
                        None => "Invalid input.".to_string(),
                    };
                    Paragraph::new(Text::from(msg))
                        .block(Block::default().title("Output").borders(Borders::ALL))
                }
                _ => {
                    Paragraph::new("Unexpected state").block(Block::default().borders(Borders::ALL))
                }
            };

            f.render_widget(widget, chunks[step]);
        })?;

        if event::poll(std::time::Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Esc => break,
                    KeyCode::Backspace => match step {
                        0 => {
                            input_base.pop();
                        }
                        1 => {
                            input_rate.pop();
                        }
                        2 => {}
                        _ => {}
                    },
                    KeyCode::Enter => match step {
                        0 => step = 1,
                        1 => step = 2,
                        2 => {
                            let selected = G10_CURRENCIES
                                .get(selected_currency_idx)
                                .unwrap_or(&"EUR")
                                .to_string();

                            result = apply_percentage_str(&input_base, &input_rate, &selected);
                            step = 3;
                        }
                        3 => break,
                        _ => {}
                    },
                    KeyCode::Char(c)
                        if (c.is_ascii_digit() || c == '.') && (step == 0 || step == 1) =>
                    {
                        match step {
                            0 => input_base.push(c),
                            1 => input_rate.push(c),
                            _ => {}
                        }
                    }
                    KeyCode::Up if step == 2 => {
                        if selected_currency_idx > 0 {
                            selected_currency_idx = selected_currency_idx.saturating_sub(1)
                        }
                    }
                    KeyCode::Down if step == 2 => {
                        if selected_currency_idx + 1 < G10_CURRENCIES.len() {
                            selected_currency_idx += 1;
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(())
}
