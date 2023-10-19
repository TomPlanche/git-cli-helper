///
/// # utils.rs
/// Contains the utils functions.
// Imports ================================================================================= Imports
use ansi_term::Colour::{Green, Red};
use std::path::Path;
// Functions  ===========================================================================  Functions
///
/// # read_file
/// Reads a file from a source folder.
///
/// ## Arguments
/// * `filename` - The name of the file to read
/// * `source` - The source folder
///
/// ## Returns
/// * `String` - The content of the file
pub fn read_file(path: &Path) -> String {
    // Read and return the file
    return std::fs::read_to_string(path).expect("Something went wrong reading the file");
}

///
/// # bye
/// Says bye.
///
/// ## Arguments
/// * `happy` - If the user is happy or not
///
/// ## Returns
/// * `()` - Nothing
pub fn bye(happy: Option<bool>) {
    match happy {
        Some(true) => println!("{}", Green.paint("Bye!")),
        Some(false) => println!("{}", Red.paint("Bye!")),
        None => println!("{}", Red.paint("Bye!")),
    }
}
