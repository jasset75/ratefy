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
use std::time::{Duration, Instant};

/// Helper to calculate result and store error message if any
fn calculate_result(
    base: &str,
    rate: &str,
    currency_code: &str,
) -> Result<(Decimal, String), String> {
    apply_percentage_str(base, rate, currency_code)
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
    let mut selected_currency = "EUR".to_string();
    let mut step = 0;
    let mut result: Option<Result<(Decimal, String), String>> = None;

    let mut show_currency_popup = false;
    let mut currency_group_index = 0; // 0 = G3, 1 = G10, 2 = All
    let mut currency_list_state = ListState::default();
    let mut search_query = String::new();

    let mut cursor_visible = true;
    let mut last_cursor_toggle = Instant::now();

    loop {
        if last_cursor_toggle.elapsed() >= Duration::from_millis(500) {
            cursor_visible = !cursor_visible;
            last_cursor_toggle = Instant::now();
        }
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

            let viewport = ratatui::layout::Rect {
                x,
                y,
                width,
                height,
            };
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
            let base_display = if step == 0 && cursor_visible {
                format!("{}▌", input_base)
            } else {
                input_base.clone()
            };
            let base_input = Paragraph::new(Text::from(base_display))
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
                Style::default().fg(ratatui::style::Color::White)
            };
            let rate_display = if step == 1 && cursor_visible {
                format!("{}▌", input_rate)
            } else {
                input_rate.clone()
            };
            let rate_input = Paragraph::new(Text::from(rate_display))
                .block(
                    Block::default()
                        .title("Enter percentage rate")
                        .borders(Borders::ALL),
                )
                .style(rate_style);
            f.render_widget(rate_input, chunks[1]);

            // Currency selection display (replaces List with stylized Paragraph)
            let selected_style = if step == 2 {
                Style::default()
                    .fg(ratatui::style::Color::Black)
                    .bg(ratatui::style::Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(ratatui::style::Color::White)
            };
            let arrow_icon = if show_currency_popup { "🔼" } else { "🔽" };
            let selected_label = format!("Currency: {} {}", selected_currency, arrow_icon);
            let currency_block = Paragraph::new(Text::from(selected_label).style(selected_style))
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(currency_block, chunks[2]);

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
                .style(Style::default().fg(ratatui::style::Color::White))
                .block(Block::default().title("Output").borders(Borders::ALL));
            f.render_widget(result_paragraph, chunks[4]);

            // Legend
            let legend_text =
                "TAB: next | Shift+TAB: prev | ↑↓: navigate | Enter: confirm | ESC: exit";
            let legend_paragraph = Paragraph::new(Text::from(legend_text))
                .style(Style::default().fg(ratatui::style::Color::White));
            f.render_widget(legend_paragraph, chunks[5]);

            if show_currency_popup {
                use ratatui::widgets::Clear;
                let groups = [&CurrencyGroup::G3, &CurrencyGroup::G10, &CurrencyGroup::All];
                // let currencies = groups[currency_group_index].list();
                let full_list = groups[currency_group_index].list();
                let filtered_list: Vec<String> = if search_query.is_empty() {
                    full_list.clone()
                } else {
                    full_list
                        .iter()
                        .filter(|c| c.to_lowercase().contains(&search_query.to_lowercase()))
                        .cloned()
                        .collect()
                };
                let popup_area = ratatui::layout::Rect {
                    x: viewport.x + 5,
                    y: viewport.y + 5,
                    width: 30,
                    height: 10,
                };
                // let items: Vec<ListItem> = currencies
                //     .iter()
                //     .map(|c| ListItem::new(c.to_string()))
                //     .collect();
                let items: Vec<ListItem> = if filtered_list.is_empty() {
                    vec![ListItem::new("No results")]
                } else {
                    filtered_list
                        .iter()
                        .map(|c| ListItem::new(c.to_string()))
                        .collect()
                };
                f.render_widget(Clear, popup_area);
                let group_titles = ["G3 ▶", "◀ G10 ▶", "◀ ALL"];
                let selected = currency_list_state.selected().unwrap_or(0);
                let list_len = filtered_list.len();
                let mut scroll_hint = String::new();
                if selected > 0 {
                    scroll_hint.push('↑');
                }
                if selected + 1 < list_len {
                    scroll_hint.push('↓');
                }
                // let title = format!("{} {}", group_titles[currency_group_index], scroll_hint);
                let title = format!(
                    "{} {} [{}]",
                    group_titles[currency_group_index], scroll_hint, search_query
                );
                let list = List::new(items)
                    .block(Block::default().title(title).borders(Borders::ALL))
                    .highlight_style(
                        Style::default()
                            .fg(ratatui::style::Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    );
                f.render_stateful_widget(list, popup_area, &mut currency_list_state);
            }
        })?;

        if event::poll(std::time::Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Esc => {
                        if show_currency_popup {
                            show_currency_popup = false;
                            search_query.clear();
                        } else {
                            break;
                        }
                    }
                    KeyCode::Backspace => {
                        if show_currency_popup {
                            search_query.pop();
                            currency_list_state.select(Some(0));
                        } else {
                            match step {
                                0 => {
                                    input_base.pop();
                                    let _ = calculate_result(
                                        &input_base,
                                        &input_rate,
                                        &selected_currency,
                                    );
                                }
                                1 => {
                                    input_rate.pop();
                                    let _ = calculate_result(
                                        &input_base,
                                        &input_rate,
                                        &selected_currency,
                                    );
                                }
                                _ => {}
                            }
                        }
                    }
                    KeyCode::Tab => {
                        step = (step + 1) % 3;
                    }
                    KeyCode::BackTab => {
                        step = if step == 0 { 2 } else { step - 1 };
                    }
                    KeyCode::Up => {
                        if show_currency_popup {
                            if let Some(selected) = currency_list_state.selected() {
                                if selected > 0 {
                                    currency_list_state.select(Some(selected - 1));
                                }
                            }
                        } else if step != 0 {
                            step -= 1;
                        }
                    }
                    KeyCode::Down => {
                        if show_currency_popup {
                            let selected = currency_list_state.selected().unwrap_or(0);
                            let list_len = match currency_group_index {
                                0 => {
                                    let groups = [
                                        &CurrencyGroup::G3,
                                        &CurrencyGroup::G10,
                                        &CurrencyGroup::All,
                                    ];
                                    let full_list = groups[currency_group_index].list();
                                    let filtered_list: Vec<String> = if search_query.is_empty() {
                                        full_list.clone()
                                    } else {
                                        full_list
                                            .iter()
                                            .filter(|c| {
                                                c.to_lowercase()
                                                    .contains(&search_query.to_lowercase())
                                            })
                                            .cloned()
                                            .collect()
                                    };
                                    filtered_list.len()
                                }
                                1 => {
                                    let groups = [
                                        &CurrencyGroup::G3,
                                        &CurrencyGroup::G10,
                                        &CurrencyGroup::All,
                                    ];
                                    let full_list = groups[currency_group_index].list();
                                    let filtered_list: Vec<String> = if search_query.is_empty() {
                                        full_list.clone()
                                    } else {
                                        full_list
                                            .iter()
                                            .filter(|c| {
                                                c.to_lowercase()
                                                    .contains(&search_query.to_lowercase())
                                            })
                                            .cloned()
                                            .collect()
                                    };
                                    filtered_list.len()
                                }
                                _ => {
                                    let groups = [
                                        &CurrencyGroup::G3,
                                        &CurrencyGroup::G10,
                                        &CurrencyGroup::All,
                                    ];
                                    let full_list = groups[currency_group_index].list();
                                    let filtered_list: Vec<String> = if search_query.is_empty() {
                                        full_list.clone()
                                    } else {
                                        full_list
                                            .iter()
                                            .filter(|c| {
                                                c.to_lowercase()
                                                    .contains(&search_query.to_lowercase())
                                            })
                                            .cloned()
                                            .collect()
                                    };
                                    filtered_list.len()
                                }
                            };
                            if selected + 1 < list_len {
                                currency_list_state.select(Some(selected + 1));
                            }
                        } else {
                            step = (step + 1) % 3;
                        }
                    }
                    KeyCode::Right => {
                        if show_currency_popup {
                            currency_group_index = (currency_group_index + 1) % 3;
                            currency_list_state.select(Some(0));
                            search_query.clear();
                        }
                    }
                    KeyCode::Left => {
                        if show_currency_popup && currency_group_index > 0 {
                            currency_group_index -= 1;
                            currency_list_state.select(Some(0));
                            search_query.clear();
                        }
                    }
                    KeyCode::Enter => {
                        if step == 2 && !show_currency_popup {
                            show_currency_popup = true;
                            currency_group_index = 0;
                            currency_list_state.select(Some(0));
                            search_query.clear();
                        } else if show_currency_popup {
                            let groups =
                                [&CurrencyGroup::G3, &CurrencyGroup::G10, &CurrencyGroup::All];
                            let full_list = groups[currency_group_index].list();
                            let filtered_list: Vec<String> = if search_query.is_empty() {
                                full_list.clone()
                            } else {
                                full_list
                                    .iter()
                                    .filter(|c| {
                                        c.to_lowercase().contains(&search_query.to_lowercase())
                                    })
                                    .cloned()
                                    .collect()
                            };
                            if let Some(selected) = currency_list_state.selected() {
                                if !filtered_list.is_empty() {
                                    selected_currency = filtered_list[selected].clone();
                                }
                                result = Some(calculate_result(
                                    &input_base,
                                    &input_rate,
                                    &selected_currency,
                                ));
                                search_query.clear();
                                show_currency_popup = false;
                            }
                        } else {
                            step = (step + 1) % 3;
                        }
                    }
                    KeyCode::Char(c)
                        if (c.is_ascii_digit() || c == '.') && (step == 0 || step == 1) =>
                    {
                        match step {
                            0 => {
                                input_base.push(c);
                                result = Some(calculate_result(
                                    &input_base,
                                    &input_rate,
                                    &selected_currency,
                                ));
                            }
                            1 => {
                                input_rate.push(c);
                                result = Some(calculate_result(
                                    &input_base,
                                    &input_rate,
                                    &selected_currency,
                                ));
                            }
                            _ => {}
                        }
                    }
                    KeyCode::Char(c) => {
                        if show_currency_popup {
                            search_query.push(c);
                            currency_list_state.select(Some(0));
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(())
}
