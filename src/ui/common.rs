use std::os::fd;

use ratatui::style::Color;
use ratatui::style::Style;
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

pub fn ds_list_view(app: &App) -> List<'_> {
    let mut fnds: Vec<ListItem> = vec![];
    if let States::NORMAL(tab) = app.get_state() {
        let index = usize::try_from(*tab).unwrap();
        let current_directory = app.get_directories().get(index).unwrap();
        let dirs = current_directory.get_only_dirs();
        // let files = current_directory.get_only_files();
        let dirs: Vec<ListItem> = dirs.iter().map(|f| ListItem::new(f.clone())).collect();
        // let files: Vec<ListItem> = files.iter().map(|f| ListItem::new(f.clone())).collect();
        fnds.extend(dirs);
        // fnds.extend(files);
    }
    let fnds = List::new(fnds);
    fnds
}

pub fn fs_list_view(app: &App) -> List<'_> {
    let mut fnds: Vec<ListItem> = vec![];
    if let States::NORMAL(tab) = app.get_state() {
        let index = usize::try_from(*tab).unwrap();
        let current_directory = app.get_directories().get(index).unwrap();
        // let dirs = current_directory.get_only_dirs();
        let files = current_directory.get_only_files();
        // let dirs: Vec<ListItem> = dirs.iter().map(|f| ListItem::new(f.clone())).collect();
        let files: Vec<ListItem> = files.iter().map(|f| ListItem::new(f.clone())).collect();
        // fnds.extend(dirs);
        fnds.extend(files);
    }
    let fnds = List::new(fnds);
    fnds
}

pub fn fnds_list_view(app: &App) -> List<'_> {
    let mut fnds: Vec<ListItem> = vec![];
    if let States::NORMAL(tab) = app.get_state() {
        let index = usize::try_from(*tab).unwrap();
        let current_directory = app.get_directories().get(index).unwrap();
        let dirs = current_directory.get_only_dirs();
        let files = current_directory.get_only_files();
        let dirs: Vec<ListItem> = dirs
            .iter()
            .map(|f| {
                ListItem::new(f.clone()).style(Style::default().fg(Color::Blue).bg(Color::Black))
            })
            .collect();
        let files: Vec<ListItem> = files.iter().map(|f| ListItem::new(f.clone())).collect();
        fnds.extend(dirs);
        fnds.extend(files);
    }
    let fnds = List::new(fnds);
    fnds
}
