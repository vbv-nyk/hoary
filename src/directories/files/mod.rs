pub mod file {
    use std::{
        ffi::OsString,
        fs::{DirEntry, FileType, Permissions},
        path::PathBuf,
        time::SystemTime,
    };

    pub struct File {
        name: OsString,
        path: PathBuf,
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
        pub fn new(f: &DirEntry) -> File {
            let name = f.file_name();
            let path = f.path();
            let file_type = f.file_type().unwrap();
            let is_dir = f.metadata().unwrap().is_dir();
            let is_file = f.metadata().unwrap().is_file();
            let is_symlink = f.metadata().unwrap().is_symlink();
            let length = f.metadata().unwrap().len();
            let permission = f.metadata().unwrap().permissions();
            let modified = f.metadata().unwrap().modified().unwrap();
            let accessed = f.metadata().unwrap().accessed().unwrap();
            let created = f.metadata().unwrap().created().unwrap();

            File {
                name,
                path,
                file_type,
                is_dir,
                is_file,
                is_symlink,
                accessed,
                created,
                modified,
                length,
                permission,
            }
        }

        pub fn get_name(&self) -> &OsString {
            &self.name
        }

        pub fn get_path(&self) -> &PathBuf {
            &self.path
        }

        pub fn get_file_type(&self) -> &FileType {
            &self.file_type
        }

        pub fn is_dir(&self) -> bool {
            self.is_dir
        }

        pub fn is_file(&self) -> bool {
            self.is_file
        }

        pub fn is_symlink(&self) -> bool {
            self.is_symlink
        }

        pub fn get_length(&self) -> u64 {
            self.length
        }

        pub fn get_permission(&self) -> &Permissions {
            &self.permission
        }

        pub fn get_modified_time(&self) -> &SystemTime {
            &self.modified
        }

        pub fn get_accessed_time(&self) -> &SystemTime {
            &self.accessed
        }

        pub fn get_created_time(&self) -> &SystemTime {
            &self.created
        }
    }
}
