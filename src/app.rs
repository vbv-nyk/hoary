pub mod directory;

pub mod app_mod {
    pub type TerminalType = Terminal<CrosstermBackend<Stdout>>;

    use std::io::{stdout, Stdout};

    use crossterm::{
        event::{DisableMouseCapture, EnableMouseCapture},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    };
    use ratatui::{backend::CrosstermBackend, Terminal};

    use super::directory::directory_mod::Directory;
    use anyhow::Result;

    pub enum States {
        NORMAL(usize),
    }
    pub struct App {
        pub directories: Vec<Directory>,
        pub state: States,
    }

    impl App {
        pub fn new() -> App {
            let mut directories = vec![];
            directories.push(Directory::new(Some("."), 1));
            App {
                directories,
                state: States::NORMAL(0),
            }
        }

        pub fn start(&self) -> Result<TerminalType> {
            enable_raw_mode()?;
            crossterm::execute!(stdout(), EnterAlternateScreen, EnableMouseCapture)?;
            let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
            Ok(terminal)
        }

        pub fn end(&self) -> Result<()> {
            disable_raw_mode()?;
            crossterm::execute!(stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
            Ok(())
        }

        pub fn get_open_directories(&self) -> Vec<String> {
            self.directories
                .iter()
                .map(|f| f.get_directory_name().clone())
                .collect()
        }

        pub fn get_directories_at_tab(&self) -> Vec<String> {
            if let States::NORMAL(tab_index) = self.state {
                return self.directories.get(tab_index).unwrap().get_only_dirs();
            }
            self.directories.get(0).unwrap().get_only_dirs()
        }

        pub fn get_files_at_tab(&self) -> Vec<String> {
            if let States::NORMAL(tab_index) = self.state {
                return self.directories.get(tab_index).unwrap().get_only_files();
            }
            self.directories.get(0).unwrap().get_only_files()
        }

        pub fn new_directory(&mut self, path: &str) -> &mut Directory {
            let directory = Directory::new(Some(path), self.directories.len() - 1);
            self.directories.push(directory);
            self.directories.last_mut().unwrap()
        }

        pub fn delete_directory(&mut self, position: usize) {
            self.directories.remove(position);
        }

        pub fn get_state(&self) -> &States {
            &self.state
        }
    }
}
