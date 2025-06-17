use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::Text,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};
use ratefy_lib::apply_percentage_str;
use ratefy_lib::money::CurrencyGroup;
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

            // Input base
            let base_style = if step == 0 {
                Style::default()
                    .fg(ratatui::style::Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            let base_input = Paragraph::new(Text::from(input_base.as_str()))
                .block(
                    Block::default()
                        .title("Enter base value")
                        .borders(Borders::ALL),
                )
                .style(base_style);
            f.render_widget(base_input, chunks[0]);

            // Input rate
            let rate_style = if step == 1 {
                Style::default()
                    .fg(ratatui::style::Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            let rate_input = Paragraph::new(Text::from(input_rate.as_str()))
                .block(
                    Block::default()
                        .title("Enter percentage rate")
                        .borders(Borders::ALL),
                )
                .style(rate_style);
            f.render_widget(rate_input, chunks[1]);

            // Currency list
            let items: Vec<ListItem> = CurrencyGroup::G10
                .list()
                .iter()
                .map(|c| ListItem::new(c.to_string()))
                .collect();
            let mut state = ListState::default();
            state.select(Some(selected_currency_idx));
            let currency_list = List::new(items)
                .block(
                    Block::default()
                        .title("Select currency")
                        .borders(Borders::ALL),
                )
                .highlight_style(if step == 2 {
                    Style::default().add_modifier(Modifier::REVERSED)
                } else {
                    Style::default()
                });
            f.render_stateful_widget(currency_list, chunks[2], &mut state);

            // chunks[3] left empty for spacing
            let blank = Paragraph::new("");
            f.render_widget(blank, chunks[3]);

            // Result box
            let msg = match &result {
                Some((amount, currency)) => format!("Result: {:.2} {}", amount, currency),
                None => "Result will appear here.".to_string(),
            };
            let result_paragraph = Paragraph::new(Text::from(msg))
                .block(Block::default().title("Output").borders(Borders::ALL))
                .style(if step == 3 {
                    Style::default().fg(ratatui::style::Color::Yellow)
                } else {
                    Style::default()
                });
            f.render_widget(result_paragraph, chunks[4]);
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
                            let list = CurrencyGroup::G10.list();
                            let selected = list
                                .get(selected_currency_idx)
                                .cloned()
                                .unwrap_or_else(|| "EUR".to_string());

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
                        if selected_currency_idx + 1 < CurrencyGroup::G10.list().len() {
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
