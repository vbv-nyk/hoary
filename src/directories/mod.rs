mod files;
pub mod directory {
    use std::{
        ffi::OsString,
        fs::{self, DirEntry, File},
        io::Error,
    };

    pub struct Directory {
        pub contents: Vec<DirEntry>,
        pub files: Vec<File>,
    }

    impl Directory {
        pub fn new(path: Option<&str>) -> Directory {
            let contents = init(path.unwrap_or_else(|| "."));
            Directory {
                contents,
                files: vec![],
            }
        }

        pub fn get_file_names(&self) -> Vec<OsString> {
            let file_names: Vec<OsString> =
                self.contents.iter().map(|fnd| fnd.file_name()).collect();
            for (i, file_name) in file_names.iter().enumerate() {
                println!("{i}: {:#?}", file_name);
            }
            file_names
        }

        pub fn get_file_paths(&self) -> Vec<OsString> {
            let file_paths: Vec<OsString> = self
                .contents
                .iter()
                .map(|fnd| fnd.path().into_os_string())
                .collect();

            for (i, file_path) in file_paths.iter().enumerate() {
                println!("{i}: {:#?}", file_path);
            }
            file_paths
        }
    }

    pub fn load_directories(path: &str) -> Result<Vec<DirEntry>, Error> {
        let current_directory = fs::read_dir(path)?;
        let current_directory = current_directory.into_iter();

        let fnds = current_directory.map(|fnd| fnd.unwrap());
        let fnds: Vec<DirEntry> = fnds.collect();
        Ok(fnds)
    }

    pub fn load_contents(fnds: Result<Vec<DirEntry>, Error>) -> Vec<DirEntry> {
        let fnds = match fnds {
            Ok(fnds) => fnds,
            Err(e) => panic!("Couldn't open the files and the directories because {e}"),
        };

        fnds
    }

    pub fn load_files(fnds: Result<Vec<DirEntry>, Error>) -> Vec<File> {
        let fnds = match fnds {
            Ok(fnds) => {}
            Err(e) => panic!("Couldn't open the files and the directories because {e}"),
        };
    }

    pub fn init(path: &str) -> Vec<DirEntry> {
        load_contents(load_directories(path))
    }
}
