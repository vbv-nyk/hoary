use std::error::Error;

use directories::directory::Directory;

mod directories;

fn main() -> Result<(), Box<dyn Error>> {
    let directories = Directory::new(Some("."));
    let files = directories.get_files();

    files.iter().for_each(|f| f.open());
    Ok(())
}
