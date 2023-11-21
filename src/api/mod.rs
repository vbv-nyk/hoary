pub mod directory_mod;
pub mod file_mod;

pub mod tui_controller {
    use anyhow;
    use crossterm::{
        event::{self, KeyCode, KeyEventKind},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    };
    use ratatui::{
        prelude::{CrosstermBackend, Stylize, Terminal},
        widgets::Paragraph,
    };
    use std::io::{stdout, Result, Stdout};

    pub fn startup() -> Result<(Terminal<CrosstermBackend<Stdout>>)> {
        stdout().execute(EnterAlternateScreen)?;
        enable_raw_mode()?;
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
        terminal.clear()?;
        Ok((terminal))
    }

    pub fn end() -> Result<()> {
        stdout().execute(LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    }
}
