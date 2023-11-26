pub mod directory;

pub mod app_mod {

    use std::io::{stdout, Stdout};

    use super::directory::{directory_mod::Directory, file};
    use anyhow::Result;
    use dioxus::core_macro::Props;

    pub enum States {
        NORMAL(usize),
    }

    #[derive(PartialEq, Props)]
    pub struct App {
        pub directories: Vec<Directory>,
    }

    impl App {
        pub fn new(path: Option<&str>) -> App {
            let mut directories = vec![];
            directories.push(Directory::new(path));
            App {
                directories: directories,
            }
        }

        pub fn get_directories(&self) -> Vec<Directory> {
            self.directories.to_owned()
        }

        pub fn get_open_directories(&self) -> Vec<String> {
            self.directories
                .iter()
                .map(|f| f.get_directory_name().clone())
                .collect()
        }

        pub fn get_directories_at_tab(&self, tab_index: usize) -> Vec<String> {
            self.directories
                .get(tab_index)
                .unwrap()
                .get_only_dirs()
                .iter()
                .map(|f| f.get_name().clone().into_string().unwrap())
                .collect()
        }

        pub fn get_files_at_tab(&self, tab_index: usize) -> Vec<String> {
            self.directories
                .get(tab_index)
                .unwrap()
                .get_only_files()
                .iter()
                .map(|f| f.get_name().clone().into_string().unwrap())
                .collect()
        }

        pub fn open_file_at_tab(&self, tab_index: usize, file_index: usize) {
            self.directories
                .get(tab_index)
                .unwrap()
                .get_only_files()
                .get(file_index)
                .unwrap()
                .open();
        }

        pub fn open_directory_at_tab(&self, tab_index: usize, directory_index: usize) {
            let path = self
                .directories
                .get(tab_index)
                .unwrap()
                .get_only_dirs()
                .get(directory_index)
                .unwrap()
                .get_path();
        }

        pub fn new_directory(&mut self, path: &str) -> &mut Directory {
            let directory = Directory::new(Some(path));
            self.directories.push(directory);
            self.directories.last_mut().unwrap()
        }

        pub fn delete_directory(&mut self, position: usize) {
            self.directories.remove(position);
        }

        pub fn get_path_at_index(&self, tab_index: usize, file_index: usize) -> String {
            self.get_directories()
                .get(tab_index)
                .unwrap()
                .get_files()
                .get(file_index)
                .unwrap()
                .get_path()
                .clone()
                .into_os_string()
                .into_string()
                .unwrap()
        }

        pub fn get_app_copy(&self) -> App {
            App {
                directories: self.get_directories(),
            }
        }

        pub fn change_directory(&mut self, index: usize, path: &str) -> App {
            let directories: Vec<Directory> = self
                .directories
                .iter()
                .enumerate()
                .map(|(i, f)| {
                    if i == index {
                        Directory::new(Some(path))
                    } else {
                        f.to_owned()
                    }
                })
                .collect();
            App { directories }
        }
    }
}
