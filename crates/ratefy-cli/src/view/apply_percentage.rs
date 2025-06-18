use crate::types::layout::{HorizontalAlign, VerticalAlign};
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

/// Helper to calculate result and store error message if any
fn calculate_result(
    base: &str,
    rate: &str,
    currency_idx: usize,
) -> Result<(Decimal, String), String> {
    let list = CurrencyGroup::G10.list();
    let selected = list
        .get(currency_idx)
        .cloned()
        .unwrap_or_else(|| "EUR".to_string());

    apply_percentage_str(base, rate, &selected)
        .ok_or_else(|| "Could not calculate percentage".to_string())
}

/// Handles the percentage calculation screen
pub fn apply_percentage_view(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    h_align: HorizontalAlign,
    v_align: VerticalAlign,
    show_border: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut input_base = String::new();
    let mut input_rate = String::new();
    let mut selected_currency_idx = 0;
    let mut step = 0;
    let mut result: Option<Result<(Decimal, String), String>> = None;

    loop {
        terminal.draw(|f| {
            let outer = f.size();

            let width = match h_align {
                HorizontalAlign::Full => outer.width,
                _ => 80,
            };

            let height = match v_align {
                VerticalAlign::Full => outer.height,
                _ => 20,
            };

            let x = match h_align {
                HorizontalAlign::Left | HorizontalAlign::Full => outer.x,
                HorizontalAlign::Center => outer.x + (outer.width.saturating_sub(width)) / 2,
                HorizontalAlign::Right => outer.x + outer.width.saturating_sub(width),
            };

            let y = match v_align {
                VerticalAlign::Top | VerticalAlign::Full => outer.y,
                VerticalAlign::Middle => outer.y + (outer.height.saturating_sub(height)) / 2,
                VerticalAlign::Bottom => outer.y + outer.height.saturating_sub(height),
            };

            let viewport = ratatui::layout::Rect { x, y, width, height };
            if show_border {
                let frame_block = Block::default()
                    .borders(Borders::ALL)
                    .title("Apply Percentage")
                    .style(Style::default().bg(ratatui::style::Color::Blue));
                f.render_widget(frame_block, viewport);
            }

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(1),
                    Constraint::Length(3),
                    Constraint::Length(4),
                ])
                .split(viewport);

            // Input base
            let base_style = if step == 0 {
                Style::default()
                    .fg(ratatui::style::Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(ratatui::style::Color::White)
            };
            let base_input = Paragraph::new(Text::from(input_base.as_str()))
                .block(Block::default().title("Enter base value").borders(Borders::ALL))
                .style(base_style);
            f.render_widget(base_input, chunks[0]);

            // Input rate
            let rate_style = if step == 1 {
                Style::default()
                    .fg(ratatui::style::Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(ratatui::style::Color::White)
            };
            let rate_input = Paragraph::new(Text::from(input_rate.as_str()))
                .block(Block::default().title("Enter percentage rate").borders(Borders::ALL))
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
                .block(Block::default().title("Select currency").borders(Borders::ALL))
                .highlight_style(if step == 2 {
                    Style::default()
                        .fg(ratatui::style::Color::Yellow)
                        .add_modifier(Modifier::REVERSED)
                } else {
                    Style::default().fg(ratatui::style::Color::White)
                });
            f.render_stateful_widget(currency_list, chunks[2], &mut state);

            // Spacing
            let blank = Paragraph::new("");
            f.render_widget(blank, chunks[3]);

            // Result
            let msg = match &result {
                Some(Ok((amount, currency))) => format!("Result: {:.2} {}", amount, currency),
                Some(Err(err_msg)) => format!("Error: {}", err_msg),
                None => "Result will appear here.".to_string(),
            };
            let result_paragraph = Paragraph::new(Text::from(msg))
                .block(Block::default().title("Output").borders(Borders::ALL))
                .style(if step == 3 {
                    Style::default().fg(ratatui::style::Color::Yellow)
                } else {
                    Style::default().fg(ratatui::style::Color::White)
                });
            f.render_widget(result_paragraph, chunks[4]);

            // Legend
            let legend_text =
                "TAB: next | Shift+TAB: prev | ↑↓: navigate | Enter: confirm | ESC: exit";
            let legend_paragraph =
                Paragraph::new(Text::from(legend_text)).style(Style::default().fg(ratatui::style::Color::White));
            f.render_widget(legend_paragraph, chunks[5]);
        })?;

        if event::poll(std::time::Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Esc => break,
                    KeyCode::Backspace => match step {
                        0 => {
                            input_base.pop();
                            result = Some(calculate_result(&input_base, &input_rate, selected_currency_idx));
                        }
                        1 => {
                            input_rate.pop();
                            result = Some(calculate_result(&input_base, &input_rate, selected_currency_idx));
                        }
                        _ => {}
                    },
                    KeyCode::Tab => {
                        step = (step + 1) % 4;
                    }
                    KeyCode::BackTab => {
                        step = if step == 0 { 3 } else { step - 1 };
                    }
                    KeyCode::Up => {
                        if step == 2 && selected_currency_idx > 0 {
                            selected_currency_idx -= 1;
                            result = Some(calculate_result(&input_base, &input_rate, selected_currency_idx));
                        } else if step != 0 {
                            step -= 1;
                        }
                    }
                    KeyCode::Down => {
                        if step == 2 && selected_currency_idx + 1 < CurrencyGroup::G10.list().len() {
                            selected_currency_idx += 1;
                            result = Some(calculate_result(&input_base, &input_rate, selected_currency_idx));
                        } else {
                            step = (step + 1) % 4;
                        }
                    }
                    KeyCode::Enter => {
                        if step < 3 {
                            step += 1;
                        }
                    }
                    KeyCode::Char(c)
                        if (c.is_ascii_digit() || c == '.') && (step == 0 || step == 1) =>
                    {
                        match step {
                            0 => {
                                input_base.push(c);
                                result = Some(calculate_result(&input_base, &input_rate, selected_currency_idx));
                            }
                            1 => {
                                input_rate.push(c);
                                result = Some(calculate_result(&input_base, &input_rate, selected_currency_idx));
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(())
}