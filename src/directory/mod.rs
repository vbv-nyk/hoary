pub mod file;
pub mod directory {
    use std::{
        ffi::OsString,
        fs::{self, DirEntry},
        io::Error,
        path::PathBuf,
    };

    use super::file::file::File;

    pub struct Directory {
        files: Vec<File>,
    }

    impl Directory {
        pub fn new(path: Option<&str>) -> Directory {
            let current_directory = load_directories(path.unwrap_or("."));
            match current_directory {
                Ok(fnds) => {
                    return Directory {
                        files: load_files(&fnds),
                    }
                }
                Err(e) => panic!("Couldn't open the files and the directories because {e}"),
            };
        }
        pub fn get_files(&self) -> &Vec<File> {
            &self.files
        }

        pub fn get_file_names(&self) -> Vec<OsString> {
            let file_names: Vec<OsString> = self
                .files
                .iter()
                .map(|file| file.get_name().clone())
                .collect();
            for (i, file_name) in file_names.iter().enumerate() {
                println!("{i}: {:#?}", file_name);
            }
            file_names
        }

        pub fn get_file_paths(&self) -> Vec<PathBuf> {
            let file_paths: Vec<PathBuf> = self
                .files
                .iter()
                .map(|file| file.get_path().clone())
                .collect();

            for (i, file_path) in file_paths.iter().enumerate() {
                println!("{i}: {:#?}", file_path);
            }
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
