/// # main.rs
/// This is the main file for the project.
///
/// Read the [README.md](../README.md) for more information.
// Imports
use ansi_term::Colour::{Red, Green};
use ansi_term::Style;


/// # Main function
fn main() {
    match std::env::current_dir() {
        Ok(current_dir) => {
            // Folder caller
            let caller = current_dir;

            // Read the passed arguments
            let args: Vec<String> = std::env::args().collect();

            // Print the arguments
            println!("Arguments: {:?}", args);

            // Looks if a file named 'commit_message.txt' exists in the 'caller' folder
            if std::path::Path::new(&format!("{}/commit_message.txt", caller.display())).exists() {
                // If it exists, print a message
                println!("commit_message.txt {} ✅ ",
                    Green.bold().paint("found")
                );

                // Read the file
                let commit_message = std::fs::read_to_string(format!("{}/commit_message.txt", caller.display())).expect("Something went wrong reading the file");

                // Print the commit message
                println!("Commit message: \n{}", commit_message);

                // User Validation
                let mut user_validation = String::new();
                println!("Do you want to commit with this message? (y/n) {}",
                    Style::new().underline().paint("default: y")
                );
                std::io::stdin().read_line(&mut user_validation).expect("Failed to read line");

                // If the user wants to commit with this message (y | Y | yes | Yes | return key)
                if user_validation.trim().to_ascii_lowercase() == "y" || user_validation.trim().to_ascii_lowercase() == "yes" || user_validation.trim() == "" {
                    // Commit
                    println!("Commiting...");

                    // Command
                    let command = std::process::Command::new("git")
                        .arg("commit")
                        .arg("-m")
                        .arg(commit_message)
                        .output()
                        .expect("failed to execute process");

                    // If the command was successful
                    if command.status.success() {
                        // Print a success message
                        println!("{}",
                            Green.bold().paint("Commit successful")
                        );
                    } else {
                        // Print an error message
                        println!("{}",
                            Red.bold().paint("Commit failed")
                        );
                    }

                } else {
                    // If the user doesn't want to commit with this message
                    println!("Commit aborted");
                }

            } else {
                // If it doesn't exist, print an error message
                println!("commit_message.txt {} ❌ ",
                    Red.bold().paint("not found")
                );
            }
        },
        Err(e) => {
            println!("Failed to get current working directory: {}", e);
        }
    }

}
