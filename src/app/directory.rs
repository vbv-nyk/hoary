pub mod file;

pub mod directory_mod {
    use std::{
        env::current_dir,
        ffi::OsString,
        fs::{self, DirEntry},
        io::Error,
        path::{Path, PathBuf},
        vec,
    };

    use dioxus::core_macro::Props;

    use super::file::file_mod::File;

    enum DirectoryStatus {
        SELECTED(usize),
    }

    #[derive(PartialEq, Props, Debug, Clone)]
    pub struct Directory {
        name: String,
        files: Vec<File>,
    }
    // As any other widget, a Table can be wrapped in a Block.
    impl Directory {
        pub fn new(path: Option<&str>) -> Directory {
            let fnds = load_directories(path.unwrap_or("."));
            let path = Path::new(path.unwrap());
            let mut path = path.canonicalize().unwrap();
            let path = path
                .file_name()
                .unwrap()
                .to_os_string()
                .into_string()
                .unwrap();
            return Directory {
                name: path,
                files: load_files(&fnds.unwrap()),
            };
        }
        pub fn get_directory_name(&self) -> &String {
            &self.name
        }

        pub fn get_files(&self) -> &Vec<File> {
            &self.files
        }

        pub fn get_file_names(&self) -> Vec<String> {
            let file_names: Vec<String> = self
                .files
                .iter()
                .map(|file| file.get_name().clone().into_string().unwrap())
                .collect();

            file_names
        }

        pub fn get_only_dirs(&self) -> Vec<&File> {
            let mut dirs = vec![];
            self.files.iter().for_each(|f| {
                if f.is_dir() {
                    dirs.push(f);
                }
            });
            // dirs.sort();
            dirs
        }

        pub fn get_only_files(&self) -> Vec<&File> {
            let mut files = vec![];
            self.files.iter().for_each(|f| {
                if f.is_file() && !f.is_dir() {
                    files.push(f);
                }
            });
            // files.sort();
            files
        }

        pub fn get_file_paths(&self) -> Vec<PathBuf> {
            let file_paths: Vec<PathBuf> = self
                .files
                .iter()
                .map(|file| file.get_path().clone())
                .collect();

            file_paths
        }

        pub fn remove_file(&mut self, source: &OsString) -> Result<(), Error> {
            match fs::remove_file(source) {
                Ok(()) => {
                    self.files.retain(|f| f.get_name() != source);
                    Ok(())
                }
                Err(e) => return Err(e),
            }
        }

        pub fn remove_directory(&mut self, directory: &OsString) -> Result<(), Error> {
            match fs::remove_dir_all(directory) {
                Ok(()) => {
                    self.files.retain(|f| f.get_name() != directory);
                    Ok(())
                }
                Err(e) => {
                    println!("Could not rename the file: {e}");
                    Err(e)
                }
            }
        }

        pub fn copy(&self, source: &PathBuf, dest: &PathBuf) -> Result<u64, Error> {
            fs::copy(source, dest)
        }
    }

    fn load_directories(path: &str) -> Result<Vec<DirEntry>, Error> {
        let current_directory = fs::read_dir(path)?;
        let current_directory = current_directory.into_iter();

        let fnds = current_directory.map(|fnd| fnd.unwrap());
        let fnds: Vec<DirEntry> = fnds.collect();
        Ok(fnds)
    }

    fn load_files(fnds: &Vec<DirEntry>) -> Vec<File> {
        let mut files = vec![];
        fnds.iter().for_each(|f| {
            let file = File::new(f);
            files.push(file);
        });
        files
    }
}
