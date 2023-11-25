use std::os::fd;
use std::vec;

use ratatui::layout::Constraint;
use ratatui::style::*;
use ratatui::style::*;
use ratatui::text::Text;
use ratatui::widgets::*;

use crate::app::app_mod::App;
use crate::app::app_mod::States;

pub fn directory_tabs(app: &App) -> Tabs<'_> {
    let mut tabs: Tabs<'_> = Tabs::new(vec![""]);
    if let States::NORMAL(index) = app.get_state() {
        let directories: Vec<String> = app.get_open_directories();
        tabs = Tabs::new(directories)
            .select(usize::try_from(*index).unwrap())
            .highlight_style(
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            );
    }
    return tabs;
}

pub fn ds_list_view(app: &App) -> List<'_> {
    let mut fnds: Vec<ListItem> = vec![];
    if let States::NORMAL(tab) = app.get_state() {
        let index = usize::try_from(*tab).unwrap();
        let dirs = app.get_directories_at_tab();
        let dirs: Vec<ListItem> = dirs.iter().map(|f| ListItem::new(f.clone())).collect();
        fnds.extend(dirs);
    }
    let fnds = List::new(fnds);
    fnds
}

pub fn fs_list_view(app: &App) -> List<'_> {
    let mut fnds: Vec<ListItem> = vec![];
    if let States::NORMAL(tab) = app.get_state() {
        let index = usize::try_from(*tab).unwrap();
        let files = app.get_files_at_tab();
        let files: Vec<ListItem> = files.iter().map(|f| ListItem::new(f.clone())).collect();
        fnds.extend(files);
    }
    let fnds = List::new(fnds);
    fnds
}

pub fn fnds_list_view(app: &App) -> List<'_> {
    let mut fnds: Vec<ListItem> = vec![];
    if let States::NORMAL(tab) = app.get_state() {
        let index = usize::try_from(*tab).unwrap();
        let dirs = app.get_directories_at_tab();
        let files = app.get_files_at_tab();
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

pub fn fnds_table(app: &App) -> Table<'_> {
    let mut fnds: Vec<Row> = vec![];
    if let States::NORMAL(tab) = app.get_state() {
        let index: usize = usize::try_from(*tab).unwrap();
        let dirs: Vec<String> = app.get_directories_at_tab();
        let files: Vec<String> = app.get_files_at_tab();
        let dirs: Vec<Cell> = dirs
            .iter()
            .map(|f| {
                Cell::from(Text::from(f.clone()))
                    .add_modifier(Modifier::BOLD)
                    .style(Style::default().fg(Color::Blue).bg(Color::Black))
            })
            .collect();
        let files: Vec<Cell> = files
            .iter()
            .map(|f: &String| Cell::from(f.clone()))
            .collect();
        fnds.push(
            Row::new(dirs)
                .set_style(Style::default())
                .style(Style::default().fg(Color::White)),
        );
        fnds.push(Row::new(files));
    }
    let table = Table::new(fnds)
        .highlight_style(Style::default().bg(Color::Black))
        .highlight_spacing(HighlightSpacing::Never);

    table
}
