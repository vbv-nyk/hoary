use std::{
    error::Error,
    ffi::{OsStr, OsString},
    fs,
    path::{Path, PathBuf},
};

use directory::{directory::Directory, file::file::OpenOperation};

mod directory;

fn main() -> Result<(), Box<dyn Error>> {
    let directory = Directory::new(Some("."));
    directory.copy(
        &PathBuf::from("./src/main.rs"),
        &PathBuf::from("./temp-dest/main.rs"),
    );
    Ok(())
}
