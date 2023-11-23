pub mod directory;

pub mod app_mod {
    use ratatui::symbols::line::NORMAL;

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
        fn new() -> App {
            let mut directories = vec![];
            directories.push(Directory::new(Some("."), 1));
            App {
                directories,
                state: States::NORMAL(1),
            }
        }

        fn new_directory(&mut self, path: &str) -> &mut Directory {
            let directory = Directory::new(Some("."), self.directories.len() - 1);
            self.directories.push(directory);
            self.directories.last_mut().unwrap()
        }

        fn delete_directory(&mut self, position: usize) {
            self.directories.remove(position);
        }
    }
}
