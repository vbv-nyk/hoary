use ratatui::widgets::List;
use ratatui::widgets::ListItem;
use ratatui::widgets::Tabs;

use crate::app::app_mod::App;
use crate::app::app_mod::States;

pub fn directory_tabs(app: &App) -> Tabs<'_> {
    let directories: Vec<String> = app
        .get_directories()
        .iter()
        .map(|f| f.get_directory_name().clone())
        .collect();

    let tabs = Tabs::new(directories);
    tabs
}

pub fn fnds_list_view(app: &App) -> List<'_> {
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
