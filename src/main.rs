/// # main.rs
/// Main file for the project.
///
/// ## Arguments
/// * `-n` | `--no` - Used for directly generating the `commit_message.md` file.
/// * `-y` | `--yes` - Used for directly commiting the changes.
/// * `-v` | `--verbose` - Used for verbose the operation.
/// * `-h` | `--help` - Used for printing the help message.
///
///
/// Read the [README.md](../README.md) for more information.
// Imports ================================================================================= Imports
#[path = "./utils.rs"]
mod utils;

#[path = "git_related.rs"]
mod git_related;

use std::io::prelude::*;
use std::path::Path;

use ansi_term::Colour::{Green, Red, Yellow};
use clap::{Parser};
use dialoguer::Confirm;
use git_related::commit;

// Constants  ===========================================================================  Constants
const COMMIT_MESSAGE_FILE: &str = "commit_message.md";
const COMMITIGNORE_FILE_PATH: &str = ".commitignore";

// Cli parser
#[derive(Parser)]
#[command(about = "Custom program that can:\n\
\t- Commit with the current 'commit_message.md' file text.\n\
\t- Generates the 'commit_message.md' file.")]
#[command(author = "Tom P. <tomplanche@icloud.com>")]
#[command(help_template = "{about}\nMade by: {author}\n\nUSAGE:\n{usage}\n\n{all-args}\n")]
#[command(name = "custom-git-commit")]
struct Cli {
    /// Optional 'yes' argument.
    /// Directly commit the file with the text in `commit_message.md'.
    #[clap(short, long)]
    yes: bool,

    /// Optional 'no' argument.
    /// Generates the `commit_message.md' file.
    #[clap(short, long)]
    no: bool,

    /// Verbose the operation.
    #[arg(short, long)]
    verbose: bool,
}
// Function(s) =========================================================================== Functions
///
/// # prepare_commit_msg
/// Prepares the commit message.
/// It creates the commit message file and empty it if it already exists.
/// It also adds the modified / added files to the commit message file.
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
        std::fs::write(path, "").expect("Something went wrong emptying the file");
    } else {
        // Create the file
        std::fs::File::create(path).expect("Something went wrong creating the file");
    }

    // Read the git status
    let modified_files: Vec<String> =
        git_related::process_git_status(&git_related::read_git_status());

    // The commit message file
    let mut commit_file = std::fs::OpenOptions::new()
        .append(true) // Append to the file
        .open(path)
        .unwrap();

    let commit_number: u16 = git_related::get_current_commit_nb(None) + 1;

    if let Err(e) = writeln!(commit_file, "[{}]\n\n", commit_number) {
        eprintln!("Couldn't write to file: {}", e);
    }

    // For each modified file
    for file in modified_files {
        // If the file is not a file in the commitignore file
        // or is not in a folder in the commitignore file
        if comitignore_path.exists() {
            let commitignore_items: Vec<String> = git_related::process_gitignore_file(&path);

            if commitignore_items.contains(&file)
                && commitignore_items.contains(&format!("{}/", file))
            {
                continue;
            }
        }

        if let Err(e) = writeln!(commit_file, "- `{}`:\n\n\t\n", file) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }

    // Close the file
    commit_file.flush().unwrap();
    drop(commit_file);

    // Print a message
    println!(
        "{} {} ✅ ",
        COMMIT_MESSAGE_FILE,
        Green.bold().paint("created")
    );
}

///
/// # handle_message_exists
/// Handles the case where the commit message file already exists.
///
/// ## Arguments
/// * `caller` - The folder from which the program was called
/// * `commit_message_file_path` - The path to the commit message file
/// * `confirm` - CLI confirmation or direct commit
/// * `commit_message` - The commit message
/// * `verbose` - If the program should print messages
///
/// ## Returns
/// * `()` - Nothing
fn handle_message_exists(
    caller: &Path,
    commit_message_file_path: &Path,
    confirm: bool,
    verbose: bool,
) {
    if verbose {
        // If it exists, print a message
        println!(
            "{} {} ✅ ",
            COMMIT_MESSAGE_FILE,
            Green.bold().paint("found")
        );
    }

    let commitignore_path_str = format!("{}/{}", caller.display(), COMMITIGNORE_FILE_PATH);
    let commitignore_path = Path::new(&commitignore_path_str);

    // Read the file
    let commit_message = utils::read_file(commit_message_file_path);

    if verbose {
        if commitignore_path.exists() {
            println!(
                "{} {} ✅ ",
                COMMITIGNORE_FILE_PATH,
                Green.bold().paint("found")
            );
        } else {
            println!(
                "{} {} ❌ ",
                COMMITIGNORE_FILE_PATH,
                Yellow.bold().paint("not found")
            );
        }
    }

    // Print the commit message
    if verbose {
        print_commit_message(commit_message.clone());
    }

    if confirm {
        // User Validation
        if Confirm::new()
            .with_prompt("Do you want to commit with this message?")
            .interact()
            .unwrap()
        {
            commit(commit_message).expect("Error commiting the changes");
        } else {
            // If the user doesn't want to commit with this message
            if Confirm::new()
                .with_prompt("Do you want to edit the commit message?")
                .default(true)
                .interact()
                .unwrap()
            {
                prepare_commit_msg(commit_message_file_path);
            } else {
                // If the user doesn't want to edit the commit message
                utils::bye(None);
            }
        }
    } else {
        commit(commit_message).expect("Error commiting the changes");
    }
}

///
/// # handle_message_doesnt_exist
/// Handles the case where the commit message file doesn't exist.
///
/// ## Arguments
/// * `caller` - The folder from which the program was called
/// * `confirm` - CLI confirmation or direct creation
/// * `verbose` - If the program should print messages
///
/// ## Returns
/// * `()` - Nothing
fn handle_message_doesnt_exist(
    caller: &Path,
    confirm: bool,
    verbose: bool,
) {
    if verbose {
        // If it doesn't exist, print an error message
        println!(
            "{}/{} {} ❌ ",
            caller.display(),
            COMMIT_MESSAGE_FILE,
            Red.bold().paint("not found")
        );
    }

    if confirm {
        if Confirm::new()
            .with_prompt("Create it ?")
            .interact()
            .unwrap()
        {
            create_needed_files();
        } else {
            // If the user doesn't want to create the file
            utils::bye(None);
        }
    } else {
        create_needed_files();
    }
}

///
/// # create_needed_files
/// Creates the needed files.
///
/// ## Returns
/// * `()` - Nothing
fn create_needed_files() {
    // Create the file
    std::fs::File::create(COMMIT_MESSAGE_FILE)
        .expect("Something went wrong creating the file");

    // Prepare the commit message
    std::fs::File::create(COMMITIGNORE_FILE_PATH)
        .expect("Something went wrong creating the file");
}

///
/// # print_commit_message
/// Prints the commit message.
///
/// ## Arguments
/// * `commit_message` - The commit message
///
/// ## Returns
/// * `()` - Nothing
fn print_commit_message(commit_message: String) {
    let delimiter = "------------------------------------------------";
    println!(
        "\nCommit message: \n{}\n{}\n{}",
        delimiter, commit_message, delimiter
    );
}
// MAIN ======================================================================================= MAIN
/// # Main function
fn main() {
    // Read the passed arguments
    let args = Cli::parse();

    // Folder caller - the folder from which the program was called
    let caller = std::env::current_dir().unwrap();


    let commit_message_file_path_str = format!("{}/{}", caller.display(), COMMIT_MESSAGE_FILE);
    let commit_message_file_path = Path::new(&commit_message_file_path_str);

    // Check if the '-v' argument is passed
    let verbose: bool = args.verbose;

    if args.yes {
        if commit_message_file_path.exists() {
            // Read the file
            let commit_message = utils::read_file(commit_message_file_path);

            print_commit_message(commit_message.clone());

            // Commit the changes
            commit(commit_message).expect("Error commiting the changes");
        } else {
            // Crash the program
            panic!("{} {} ❌ ", COMMIT_MESSAGE_FILE, Red.bold().paint("not found"));
        }
    } else if args.no {
        // Create the needed files
        create_needed_files();

        // Prepare the commit message
        prepare_commit_msg(commit_message_file_path);
    } else {
        // Looks if a file named COMMIT_MESSAGE_FILE exists in the 'caller' folder
        if commit_message_file_path.exists() {
            handle_message_exists(
                &caller,
                &commit_message_file_path,
                true,
                verbose
            );
        } else {
            handle_message_doesnt_exist(
                &caller,
                true,
                verbose
            );
        }
    }
}
