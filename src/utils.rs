///
/// # utils.rs
/// This module contains the utils functions.

// Imports ================================================================================= Imports
use ansi_term::Style;
use std::path::{Path};

// Functions  ===========================================================================  Functions
///
/// # ask_user_validation
/// This function asks the user for validation (y/n).
///
/// ## Arguments
/// * `message` - The message to print
/// * `default` - The default validation, by default it's 'y'
///
/// ## Returns
/// * `bool` - If the user validated or not
pub fn ask_user_validation(message: &str, default: Option<char>) -> bool {
    let default_: char = default.unwrap_or('y');

    // Ask the user for validation
    println!("{} (y/n) {}",
        message,
        Style::new().underline().paint(format!("default: {}", default_))
    );

    // User validation
    let mut user_validation = String::new();
    std::io::stdin().read_line(&mut user_validation).expect("Failed to read line");

    // If the user validated
    if user_validation.trim().to_ascii_lowercase() == "y" || user_validation.trim().to_ascii_lowercase() == "yes" || (user_validation.trim() == "" && default_ == 'y') {
        return true;
    }

    return false;
}

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
    let content: String = std::fs::read_to_string(path)
        .expect("Something went wrong reading the file");

    // Return the file
    return content;
}
