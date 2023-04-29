///
/// # git_related
/// This module contains functions related to git.

// Imports ================================================================================= Imports


use ansi_term::Colour::{Red};
use std::path::Path;
use crate::utils::read_file;

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
/// # process_gitignore_file
/// This function processes the gitignore file.
///
/// ## Arguments
/// * `path` - The path to the gitignore file
pub fn process_gitignore_file(path: &Path) -> Vec<String> {
    // Read the gitignore file
    let gitignore_file = &read_file(&path);

    // The gitignore file stores the files and folders to ignore
    // Each line is a file or folder to ignore
    // The '#' character is used to comment a line

    // Regex to match the files and folders to ignore
    let regex_rule = regex::Regex::new(r"^[^#](.*)$").unwrap();

    // Create a vector to store the files and folders to ignore while parsing the gitignore file
    let mut files_to_ignore: Vec<String> = Vec::new();

    // For each line in the gitignore file
    for line in gitignore_file.lines() {
        // If the line matches the regex
        if regex_rule.is_match(line) {
            // Add the file or folder name to the vector
            files_to_ignore.push(line.to_string());
        }

    }

    return files_to_ignore;
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
