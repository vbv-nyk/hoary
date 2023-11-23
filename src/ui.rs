use std::io::stderr;

use ratatui::{backend::CrosstermBackend, Terminal};

mod startup_view;
use crate::app::app_mod::{App, States, TerminalType};
use anyhow::Result;

use self::startup_view::startup_view;

pub fn render(app: &mut App, terminal: &mut TerminalType) -> Result<()> {
    let state = app.get_state();

    match state {
        States::NORMAL(active_tab) => startup_view(terminal, app),
    }?;
    Ok(())
}
