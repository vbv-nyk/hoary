use std::{fs, io};

fn main() {
    let mut file_name: String = String::from(" ");
    let result = fs::copy("./src/main.rs", "./temp-dest/main.rs");
    match result {
        Ok(res) => println!("Moves {res} bytes from the source to the destination"),
        Err(e) => println!("The following error occurred {e}"),
    }
}
