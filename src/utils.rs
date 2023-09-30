use ansi_term::Colour::{Green, Red};
///
/// # utils.rs
/// This module contains the utils functions.
// Imports ================================================================================= Imports
use std::path::Path;
// Functions  ===========================================================================  Functions
///
/// # read_file
/// This function reads a file from a source folder.
///
/// ## Arguments
/// * `filename` - The name of the file to read
/// * `source` - The source folder
///
/// ## Returns
/// * `String` - The content of the file
pub fn read_file(path: &Path) -> String {
    // Read the file
    // Return the file
    return std::fs::read_to_string(path).expect("Something went wrong reading the file");
}

///
/// # bye
/// This function says bye.
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
