/// # main.rs
/// This is the main file for the project.
///
/// Read the [README.md](../README.md) for more information.

// Imports ================================================================================= Imports
mod utils;
mod git_related;

use std::io::prelude::*;
use std::path::Path;

use dialoguer::{Confirm, MultiSelect};
use ansi_term::Colour::{Red, Green};

// Constants  ===========================================================================  Constants
const COMMIT_MESSAGE_FILE: &str = "commit_message.txt";
const COMMITIGNORE_FILE_PATH: &str = ".commitignore";

// Function(s) =========================================================================== Functions
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
fn prepare_commit_msg(path: &Path) {
    // Get the location of the file passed by 'path'
    // ex: path = /home/user/project/src/main.rs
    // get the location of the file: /home/user/project/src/
    let folder_path = path.parent().unwrap();

    // Get the path to the commit message file
    let comitignore_path = folder_path.join(COMMITIGNORE_FILE_PATH);


    // If the COMMIT_MESSAGE_FILE exists
    if path.exists() {
        // Empty the file
        std::fs::write(path, "")
            .expect("Something went wrong emptying the file");
    } else {
        // Create the file
        std::fs::File::create(path)
            .expect("Something went wrong creating the file");
    }

    // Read the git status
    let modified_files: Vec<String> = git_related::process_git_status(&git_related::read_git_status());




    // The commit message file
    let mut commit_file = std::fs::OpenOptions::new()
        .append(true) // Append to the file
        .open(path)
        .unwrap();

    let commit_number: u8 = git_related::get_current_commit_nb(None) + 1;


    if let Err(e) = writeln!(commit_file, "[{}]\n\n", commit_number) {
        eprintln!("Couldn't write to file: {}", e);
    }

    // For each modified file
    for file in modified_files {
        // If the file is not a file in the commitignore file
        // or is not in a folder in the commitignore file
        if comitignore_path.exists() {
            let commitignore_items: Vec<String> = git_related::process_gitignore_file(&path);

            if commitignore_items.contains(&file) &&
                commitignore_items.contains(&format!("{}/", file)) {
                continue
            }
        }

        if let Err(e) = writeln!(commit_file, "- {}:\n\t\n", file) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }
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

    let commit_message_file_path_str = format!("{}/{}", caller.display(), COMMIT_MESSAGE_FILE);
    let commit_message_file_path = Path::new(&commit_message_file_path_str);

    // Looks if a file named COMMIT_MESSAGE_FILE exists in the 'caller' folder
    if commit_message_file_path.exists() {
        // If it exists, print a message
        println!("{} {} ✅ ",
            COMMIT_MESSAGE_FILE,
            Green.bold().paint("found")
        );

        // Read the file
        let commit_message = utils::read_file(commit_message_file_path);

        // Print the commit message
        let delimiter = "------------------------------------------------";
        println!("\nCommit message: \n{}\n{}\n{}", delimiter, commit_message, delimiter);

        // User Validation
        if Confirm::new()
            .with_prompt("Do you want to commit with this message?")
            .interact()
            .unwrap() {
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
            if Confirm::new()
                .with_prompt("Do you want to edit the commit message?")
                .default(true)
                .interact()
                .unwrap() {
                prepare_commit_msg(commit_message_file_path);
            } else {
                // If the user doesn't want to edit the commit message
                utils::bye(None);
            }
        }

    } else {
        // If it doesn't exist, print an error message
        println!("{}/{} {} ❌ ",
            caller.display(),
            COMMIT_MESSAGE_FILE,
            Red.bold().paint("not found")
        );

        if Confirm::new()
            .with_prompt("Create it ?")
            .interact()
            .unwrap() {
            // Create the file
            std::fs::File::create(COMMIT_MESSAGE_FILE)
                .expect("Something went wrong creating the file");

            // Prepare the commit message
            std::fs::File::create(COMMITIGNORE_FILE_PATH)
                .expect("Something went wrong creating the file");
        } else {
            // If the user doesn't want to create the file
            utils::bye(None);
        }
    }

}
