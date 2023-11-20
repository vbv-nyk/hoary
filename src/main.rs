use std::{error::Error, fs};

use directories::directory::Directory;

mod directories;

fn main() -> Result<(), Box<dyn Error>> {
    let directories = Directory::new(Some("."));
    let files = directories.get_files();
    Ok(())
}
