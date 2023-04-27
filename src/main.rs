/// # main.rs
/// This is the main file for the project.
///
/// Read the [README.md](../README.md) for more information.

// Imports ================================================================================= Imports
use std::path::Display;

use ansi_term::Colour::{Red, Green};
use ansi_term::Style;

use std::io::prelude::*;
// Constants  ===========================================================================  Constants
const COMMIT_MESSAGE_FILE: &str = "commit_message.txt";

// Function(s) =========================================================================== Functions
///
/// # read_git_status
/// This function reads the git status.
///
/// ## Returns
/// * `String` - The git status
fn read_git_status() -> String {
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
fn process_git_status(message: &String) -> Vec<String> {
    // Modified files are indicated by a 'M' at the beginning of the line
    // Added files are indicated by a 'A' at the beginning of the line

    // Regex to match the modified files and the added files
    let regex_rule = regex::Regex::new(r"^\s?[A|M]\s+(.*)$").unwrap();

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
/// # prepare_commit_msg
/// This function prepares the commit message.
/// It will create the commit message file and empty it if it already exists.
/// It will also add the modified / added files to the commit message file.
///
/// ## Arguments
/// * `source` - The source folder
///
/// ## Returns
/// * `()` - Nothing
fn prepare_commit_msg(source: &Display) {
    // If the COMMIT_MESSAGE_FILE exists
    if file_exists(COMMIT_MESSAGE_FILE, source) {
        // Empty the file
        std::fs::write(format!("{}/{}", source, COMMIT_MESSAGE_FILE), "")
            .expect("Something went wrong emptying the file");
    } else {
        // Create the file
        std::fs::File::create(format!("{}/{}", source, COMMIT_MESSAGE_FILE))
            .expect("Something went wrong creating the file");
    }

    // Read the git status
    let modified_files: Vec<String> = process_git_status(&read_git_status());

    // The commit message file
    let mut commit_file = std::fs::OpenOptions::new()
        .append(true) // Append to the file
        .open(format!("{}/{}", source, COMMIT_MESSAGE_FILE))
        .unwrap();

    if let Err(e) = writeln!(commit_file, "\n\n") {
        eprintln!("Couldn't write to file: {}", e);
    }

    // For each modified file
    for file in modified_files {
        if let Err(e) = writeln!(commit_file, "- {}:\n\t\n", file) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }
}

///
/// # file_exists
/// This function checks if a file exists.
///
/// ## Arguments
/// * `filename` - The name of the file to check
/// * `source` - The source folder
///
/// ## Returns
/// * `bool` - If the file exists or not
fn file_exists(filename: &str, source: &Display) -> bool {
    return std::path::Path::new(&format!("{}/{}", source, filename )).exists();
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
fn read_file(filename: &str, source: &Display) -> String {
    // Read the file
    let content: String = std::fs::read_to_string(format!("{}/{}", source, filename))
        .expect("Something went wrong reading the file");

    // Return the file
    return content;
}

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
fn ask_user_validation(message: &str, default: Option<char>) -> bool {
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

// MAIN ======================================================================================= MAIN
/// # Main function
fn main() {
    // Read the passed arguments
    let mut args: Vec<String> = std::env::args().collect();

    // Remove the two first arguments (the program name and the path to the program)
    args.remove(0);

    // Folder caller - the folder from which the program was called
    let caller = std::env::current_dir().unwrap();


    // Looks if a file named COMMIT_MESSAGE_FILE exists in the 'caller' folder
    if file_exists(COMMIT_MESSAGE_FILE, &caller.display()) {
        // If it exists, print a message
        println!("{} {} ✅ ",
            COMMIT_MESSAGE_FILE,
            Green.bold().paint("found")
        );

        // Read the file
        let commit_message = read_file(COMMIT_MESSAGE_FILE, &caller.display());

        // Print the commit message
        let delimiter = "------------------------------------------------";
        println!("\nCommit message: \n{}\n{}\n{}", delimiter, commit_message, delimiter);

        // User Validation
        if ask_user_validation("Do you want to commit with this message?", Some('y')) {
            // Commit
            println!("\nCommiting...");

            // Command
            let command = std::process::Command::new("git")
                .arg("commit")
                // If the args are not empty, pass them to the command
                .args(&args)
                .arg("-m")
                .arg(commit_message)
                .output()
                .expect("failed to execute process");

            // If the command was successful
            if command.status.success() {
                // Print a success message
                println!("{}",
                    Green.bold().paint("Commit successful.")
                );
            } else {
                // Print an error message
                println!("{}",
                    Red.bold().paint("Commit failed.")
                );
            }

        } else {
            // If the user doesn't want to commit with this message
            if ask_user_validation("Do you want to edit the commit message?", Some('y')) {
                prepare_commit_msg(&caller.display());
            }
        }

    } else {
        // If it doesn't exist, print an error message
        println!("{}/{} {} ❌ ",
            caller.display(),
            COMMIT_MESSAGE_FILE,
            Red.bold().paint("not found")
        );
    }

}
