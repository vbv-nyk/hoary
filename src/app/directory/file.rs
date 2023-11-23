pub mod file_mod {
    use std::{
        ffi::OsString,
        fs::{self, DirEntry, FileType, Metadata, Permissions},
        path::PathBuf,
        time::SystemTime,
    };

    pub enum OpenOperation<'a> {
        FILEOPEN,
        DIROPEN(&'a PathBuf),
    }

    pub struct File {
        name: OsString,
        path: PathBuf,
        file_type: FileType,
        is_dir: bool,
        is_file: bool,
        is_symlink: bool,
        length: u64,
        permissions: Permissions,
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
            let permissions = f.metadata().unwrap().permissions();
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
                permissions,
            }
        }

        fn parse_metadata(&mut self, metadata: Metadata) {
            self.set_is_dir(metadata.is_dir());
            self.set_is_file(metadata.is_file());
            self.set_is_symlink(metadata.is_symlink());
            self.set_length(metadata.len());
            self.set_permissions(metadata.permissions());
            self.set_modified(metadata.modified().unwrap());
            self.set_accessed(metadata.accessed().unwrap());
            self.set_created(metadata.created().unwrap());
        }

        fn set_path(&mut self, path: PathBuf) {
            self.path = path
        }

        fn set_name(&mut self, name: OsString) {
            self.name = name
        }

        fn set_accessed(&mut self, system_time: SystemTime) {
            self.accessed = system_time
        }
        fn set_created(&mut self, system_time: SystemTime) {
            self.created = system_time
        }

        fn set_modified(&mut self, system_time: SystemTime) {
            self.modified = system_time
        }

        fn set_permissions(&mut self, permissions: Permissions) {
            self.permissions = permissions
        }

        fn set_length(&mut self, length: u64) {
            self.length = length
        }

        fn set_is_symlink(&mut self, is_symlink: bool) {
            self.is_symlink = is_symlink;
        }

        fn set_is_file(&mut self, is_file: bool) {
            self.is_file = is_file;
        }

        fn set_is_dir(&mut self, is_dir: bool) {
            self.is_dir = is_dir;
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
            &self.permissions
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

        pub fn open(&self) -> OpenOperation {
            if self.is_file() {
                opener::open(self.get_path());
                OpenOperation::FILEOPEN
            } else {
                OpenOperation::DIROPEN(self.get_path())
            }
        }

        pub fn update_metadata(&mut self) {
            fs::symlink_metadata(&mut self.path);
        }

        pub fn rename_file(&mut self, new_name: OsString) {
            match fs::rename(self.get_path(), &new_name) {
                Ok(()) => {
                    self.set_name(new_name);
                }
                Err(e) => {
                    println!("Could not rename the file: {e}")
                }
            }
        }
    }
}
