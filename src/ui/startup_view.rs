use std::vec;

use crate::app::app_mod::{App, TerminalType};
use anyhow::Result;
use ratatui::{
    layout::*,
    style::{Modifier, Style},
    symbols,
    widgets::{Block, Borders},
};

use super::common::{directory_tabs, ds_list_view, fs_list_view};

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
        let ds = ds_list_view(app)
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().add_modifier(Modifier::BOLD));
        let fs = fs_list_view(app)
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().add_modifier(Modifier::BOLD));
        frame.render_widget(tabs, layout[0]);
        frame.render_widget(ds, layout[1]);
        frame.render_widget(fs, layout[2]);
    })?;

    Ok(())
}
