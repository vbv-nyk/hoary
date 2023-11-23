use std::io::Stderr;

use crate::app::app_mod::{App, TerminalType};
use anyhow::Result;
use ratatui::layout::*;
use ratatui::style::*;
use ratatui::symbols;
use ratatui::widgets::*;
use ratatui::{backend::CrosstermBackend, widgets::Tabs, Terminal};
pub fn tabs(terminal: &mut TerminalType, app: &mut App) -> Result<()> {
    let directories: Vec<String> = app
        .get_directories()
        .iter()
        .map(|f| f.get_directory_name().clone())
        .collect();

    terminal.draw(|frame| {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(20),
                Constraint::Length(20),
            ])
            .split(frame.size());

        let tabs =
            Tabs::new(directories).block(Block::default().title("Tabs").borders(Borders::ALL));

        frame.render_widget(tabs, layout[0]);
    })?;
    Ok(())
}
