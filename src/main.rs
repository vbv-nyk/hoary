use std::error::Error;

use directories::directory::Directory;

mod directories;

fn main() -> Result<(), Box<dyn Error>> {
    let directories = Directory::new(Some("."));
    directories.get_file_names();
    directories.get_file_paths();
    Ok(())
}
