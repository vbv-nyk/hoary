pub mod file {
    use std::{
        fs::{FileType, Permissions},
        time::SystemTime,
    };

    struct File {
        name: String,
        file_type: FileType,
        is_dir: bool,
        is_file: bool,
        is_symlink: bool,
        length: u64,
        permission: Permissions,
        modified: SystemTime,
        accessed: SystemTime,
        created: SystemTime,
    }

    impl File {
        fn new() -> File {}
    }
}
