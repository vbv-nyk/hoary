use std::io::Stderr;
use std::vec;

use crate::app::app_mod::States;
use crate::app::app_mod::{App, TerminalType};
use anyhow::Result;
use ratatui::layout::*;
use ratatui::widgets::*;

use super::common::{directory_tabs, fnds_list_view};
pub fn startup_view(terminal: &mut TerminalType, app: &mut App) -> Result<()> {
    let mut fnds: Vec<ListItem> = vec![];

    terminal.draw(|frame| {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(20),
                Constraint::Length(20),
            ])
            .split(frame.size());

        frame.render_widget(directory_tabs(app), layout[0]);
        frame.render_widget(fnds_list_view(app), layout[1]);
    })?;

    Ok(())
}
