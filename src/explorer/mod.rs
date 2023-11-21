pub mod directory;

mod explorer {
    use super::directory::directory::Directory;

    struct Explorer {
        directories: Vec<Directory>,
    }

    impl Explorer {
        pub fn new() -> Explorer {
            Explorer {
                directories: vec![],
            }
        }

        pub fn new_directory(&mut self, directory: Directory) {
            self.directories.push(directory)
        }
    }
}
