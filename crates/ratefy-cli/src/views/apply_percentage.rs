use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph},
};
use ratefy_lib::apply_percentage_str;
use rust_decimal::Decimal;
use std::io;

/// Handles the percentage calculation screen
pub fn apply_percentage_view(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut input_base = String::new();
    let mut input_rate = String::new();
    let mut input_currency = String::new();
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
                2 => Paragraph::new(Text::from(input_currency.as_str()))
                    .block(
                        Block::default()
                            .title("Enter currency code")
                            .borders(Borders::ALL),
                    )
                    .style(Style::default().add_modifier(Modifier::BOLD)),
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
                        2 => {
                            input_currency.pop();
                        }
                        _ => {}
                    },
                    KeyCode::Enter => match step {
                        0 => step = 1,
                        1 => step = 2,
                        2 => {
                            result =
                                apply_percentage_str(&input_base, &input_rate, &input_currency);
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
                    KeyCode::Char(c)
                        if c.is_ascii_alphanumeric() && c.is_ascii_uppercase() && step == 2 =>
                    {
                        input_currency.push(c);
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(())
}
