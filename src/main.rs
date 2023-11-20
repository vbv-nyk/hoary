use std::{
    error::Error,
    ffi::{OsStr, OsString},
    fs,
    path::PathBuf,
};

use directories::directory::Directory;

mod directories;

fn main() -> Result<(), Box<dyn Error>> {
    Ok(())
}
