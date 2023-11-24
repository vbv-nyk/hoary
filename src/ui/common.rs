use ratatui::layout::*;
use ratatui::style::*;
use ratatui::symbols;

use ratatui::widgets::Block;
use ratatui::widgets::Borders;
use ratatui::widgets::List;
use ratatui::widgets::ListItem;
use ratatui::{backend::CrosstermBackend, widgets::Tabs, Terminal};

use crate::app::app_mod::App;
use crate::app::app_mod::States;

pub fn directory_tabs(app: &mut App) -> Tabs<'_> {
    let directories: Vec<String> = app
        .get_directories()
        .iter()
        .map(|f| f.get_directory_name().clone())
        .collect();

    let tabs = Tabs::new(directories).block(Block::default().title("Tabs").borders(Borders::ALL));
    tabs
}

pub fn fnds_list_view(app: &mut App) -> List<'_> {
    let mut fnds: Vec<ListItem> = vec![];
    if let States::NORMAL(tab) = app.get_state() {
        let index = usize::try_from(*tab).unwrap();
        fnds = app
            .get_directories()
            .get(index)
            .unwrap()
            .get_file_names()
            .iter()
            .map(|f| ListItem::new(f.clone()))
            .collect();
    }
    let fnds = List::new(fnds);
    fnds
}
