///
/// # git_related
/// This module contains functions related to git.

// Imports ================================================================================= Imports
use ansi_term::Colour::{Red};

// Functions  ===========================================================================  Functions
///
/// # get_current_commit_nb
/// This function returns the number of commits.
///
/// ## Arguments
/// * `branch` - The branch to check - optional - (default: HEAD)
///
/// ## Returns
/// * `u8` - The number of commits
pub fn get_current_commit_nb(branch: Option<&str>) -> u8 {
    let output = std::process::Command::new("git")
        .arg("rev-list")
        .arg("--count")
        .arg(branch.unwrap_or("HEAD"))
        .output()
        .expect("failed to execute process");

    let output = String::from_utf8_lossy(&output.stdout);

    let output = output.trim();

    let output = output.parse::<u8>().unwrap();

    return output;
}

///
/// # process_git_status
/// This function processes the git status.
/// It will parse the git status in order to prepare the git commit message.
///
/// ## Arguments
/// * `message` - The git status
///
/// ## Returns
/// * `Vec<String>` - The modified / added files
pub fn process_git_status(message: &String) -> Vec<String> {
    // Modified files are indicated by a 'M' at the beginning of the line
    // Added files are indicated by a 'A' at the beginning of the line

    // Regex to match the modified files and the added files
    let regex_rule = regex::Regex::new(r"^[A|M]\s+(.*)$").unwrap();

    // Create a vector to store the modified / added files while parsing the git status message
    let mut modified_files: Vec<String> = Vec::new();

    // For each line in the git status message
    for line in message.lines() {
        // If the line matches the regex
        if regex_rule.is_match(line) {
            // Get the file name
            let file_name = regex_rule.captures(line).unwrap().get(1).unwrap().as_str();

            // Add the file name to the vector
            modified_files.push(file_name.to_string());
        }
    }

    // Return the vector
    return modified_files;
}

///
/// # read_git_status
/// This function reads the git status.
///
/// ## Returns
/// * `String` - The git status
pub fn read_git_status() -> String {
    // Command
    let command = std::process::Command::new("git")
        .arg("status")
        .arg("--porcelain")
        .output()
        .expect("failed to execute process");

    // If the command was successful
    return if command.status.success() {
        // Convert the output to a string
        let output = String::from_utf8_lossy(&command.stdout);

        output.to_string()
    } else {
        // Print an error message
        println!("{}",
                 Red.bold().paint("Failed to read git status.")
        );

        String::from("")
    }
}
