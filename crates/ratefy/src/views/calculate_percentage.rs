use std::io;
use crossterm::{event::{self, Event, KeyCode}};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

use ratefy_lib::calculate_percentage;

/// Handles the percentage calculation screen
pub fn calculate_percentage_view(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>
) -> Result<(), Box<dyn std::error::Error>> {
    let mut input_base = String::new();
    let mut input_rate = String::new();
    let mut step = 0;
    let mut result: Option<f64> = None;

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(5)
                .constraints([Constraint::Length(3), Constraint::Length(3), Constraint::Length(3)])
                .split(size);

            let widget = match step {
                0 => Paragraph::new(Text::from(input_base.as_str()))
                    .block(Block::default().title("Enter base value").borders(Borders::ALL))
                    .style(Style::default().add_modifier(Modifier::BOLD)),
                1 => Paragraph::new(Text::from(input_rate.as_str()))
                    .block(Block::default().title("Enter percentage rate").borders(Borders::ALL))
                    .style(Style::default().add_modifier(Modifier::BOLD)),
                2 => {
                    let msg = match result {
                        Some(val) => format!("Result: {:.2}", val),
                        None => "Invalid input.".to_string(),
                    };
                    Paragraph::new(Text::from(msg))
                        .block(Block::default().title("Output").borders(Borders::ALL))
                }
                _ => Paragraph::new("Unexpected state")
                        .block(Block::default().borders(Borders::ALL)),
            };

            f.render_widget(widget, chunks[step]);
        })?;

        if event::poll(std::time::Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Esc => break,
                    KeyCode::Backspace => match step {
                        0 => { input_base.pop(); }
                        1 => { input_rate.pop(); }
                        _ => {}
                    },
                    KeyCode::Enter => match step {
                        0 => step = 1,
                        1 => {
                            let base = input_base.trim().parse::<f64>();
                            let rate = input_rate.trim().parse::<f64>();
                            result = match (base, rate) {
                                (Ok(b), Ok(r)) => Some(calculate_percentage(b, r)),
                                _ => None,
                            };
                            step = 2;
                        }
                        2 => break,
                        _ => {}
                    },
                    KeyCode::Char(c) if c.is_ascii_digit() || c == '.' => match step {
                        0 => input_base.push(c),
                        1 => input_rate.push(c),
                        _ => {}
                    },
                    _ => {}
                }
            }
        }
    }

    Ok(())
}
