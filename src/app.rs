pub mod directory;

pub mod app_mod {
    pub type TerminalType = Terminal<CrosstermBackend<Stdout>>;

    use std::io::{stderr, stdout, Stderr, Stdout};

    use crossterm::{
        event::{DisableMouseCapture, EnableMouseCapture},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    };
    use ratatui::{backend::CrosstermBackend, symbols::line::NORMAL, Terminal};

    use super::directory::{self, directory_mod::Directory};
    use anyhow::Result;

    pub enum States {
        NORMAL(i32),
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

        pub fn start(&self) -> Result<(TerminalType)> {
            enable_raw_mode()?;
            crossterm::execute!(stdout(), EnterAlternateScreen, EnableMouseCapture)?;
            let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
            Ok((terminal))
        }

        pub fn end(&self) -> Result<()> {
            disable_raw_mode()?;
            crossterm::execute!(stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
            Ok(())
        }

        pub fn get_directories(&self) -> &Vec<Directory> {
            &self.directories
        }

        pub fn new_directory(&mut self, path: &str) -> &mut Directory {
            let directory = Directory::new(Some("."), self.directories.len() - 1);
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
