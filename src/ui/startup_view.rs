use std::vec;

use crate::app::app_mod::{App, TerminalType};
use anyhow::Result;
use ratatui::{
    layout::*,
    style::{Modifier, Style},
    symbols,
    widgets::{Block, Borders},
};

use super::common::{directory_tabs, fnds_table};

pub fn startup_view(terminal: &mut TerminalType, app: &App) -> Result<()> {
    terminal.draw(|frame| {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(10),
                Constraint::Length(20),
            ])
            .split(frame.size());

        let tabs = directory_tabs(app)
            .divider("|")
            .block(Block::default().borders(Borders::ALL));
        let fnds = fnds_table(app)
            .block(Block::default().borders(Borders::ALL))
            .widths(&[
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ]);

        frame.render_widget(tabs, layout[0]);
        frame.render_widget(fnds, layout[1]);
    })?;

    Ok(())
}
