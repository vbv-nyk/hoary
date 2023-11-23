use std::io::stderr;

use crossterm::terminal::{
    self, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};

use anyhow::Result;
use ratatui::{backend::CrosstermBackend, Terminal};

mod app;
mod event;
mod ui;
fn main() -> Result<()> {
    enable_raw_mode()?;
    crossterm::execute!(stderr(), EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stderr()));
    crossterm::execute!(stderr(), LeaveAlternateScreen)?;

    disable_raw_mode()?;
    Ok(())
}
