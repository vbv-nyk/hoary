use std::io::stderr;

use ratatui::{backend::CrosstermBackend, Terminal};

mod tabs;
use crate::app::app_mod::{App, States, TerminalType};
use anyhow::Result;

use self::tabs::tabs;

pub fn render(app: &mut App, terminal: &mut TerminalType) -> Result<()> {
    let state = app.get_state();

    match state {
        States::NORMAL(active_tab) => tabs(terminal, app),
    }?;
    Ok(())
}
