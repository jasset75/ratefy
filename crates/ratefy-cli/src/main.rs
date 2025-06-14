use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

use crate::views::calculate_percentage::calculate_percentage_view;
use ratefy_menu::{run_menu, MenuItem};

mod views;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Screen {
    MainMenu,
    CalculatePercentage,
    Exit,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut current_screen = Screen::MainMenu;

    loop {
        match current_screen {
            Screen::MainMenu => {
                let items = vec![
                    MenuItem::new(
                        "Calculate Percentage",
                        1,
                        Some("→ base * rate / 100"),
                        Screen::CalculatePercentage,
                    ),
                    MenuItem::new("Exit", 2, None, Screen::Exit),
                ];
                current_screen = run_menu(&mut terminal, "Ratefy Menu", &items)?;
            }
            Screen::CalculatePercentage => {
                calculate_percentage_view(&mut terminal)?;
                current_screen = Screen::MainMenu;
            }
            Screen::Exit => break,
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}
