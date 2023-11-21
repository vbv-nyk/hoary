use std::{
    error::Error,
    ffi::{OsStr, OsString},
    fs,
    path::PathBuf,
};

use directory::{directory::Directory, file::file::OpenOperation};

mod directory;

fn main() -> Result<(), Box<dyn Error>> {
    Ok(())
}
