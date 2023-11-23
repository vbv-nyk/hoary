use std::io::{stderr, stdout};

use app::app_mod::App;
use crossterm::terminal::{
    self, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};

use anyhow::Result;
use event::events;
use ratatui::{backend::CrosstermBackend, Terminal};
use ui::render;
mod app;
mod event;
mod helpers;
mod ui;
fn main() -> Result<()> {
    let mut app = App::new();
    app.start()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    loop {
        render(&mut app, &mut terminal)?;
        let quit = events(&mut app)?;
        if quit {
            break;
        }
    }
    app.end()?;
    Ok(())
}
