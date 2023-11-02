/// # main.rs
/// Main file for the project.
///
/// ## Arguments
/// * `-g` | `--generate` - Used for directly generating the `commit_message.md` file.
/// * `-c` | `--commit` - Used for directly commiting the changes.
///     + -p | --push - Used for directly pushing the changes.
///     + [git push args] - The args that will be passed to `git push` if the `-p` argument is passed.
/// * `-v` | `--verbose` - Used for verbose the operation.
///
///
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

use ansi_term::Colour::{Green, Red};
use clap::{Parser};
use git_related::commit;
use crate::utils::check_for_file_in_folder;

// Constants  ===========================================================================  Constants
const COMMIT_MESSAGE_FILE: &str = "commit_message.md";
const COMMITIGNORE_FILE_PATH: &str = ".commitignore";

// Args commands


// Cli parser
#[derive(Parser)]
#[command(about = "Custom program that can:\n\
\t- Commit with the current 'commit_message.md' file text.\n\
\t- Generates the 'commit_message.md' file.")]
#[command(author = "Tom P. <tomplanche@icloud.com>")]
#[command(help_template = "{about}\nMade by: {author}\n\nUSAGE:\n{usage}\n\n{all-args}\n")]
#[command(name = "custom-git-commit")]
struct Cli {
    /// Optional 'commit' argument.
    /// Directly commit the file with the text in `commit_message.md'.
    #[arg(short, long)]
    commit: bool,

    // If the '-p' argument is passed
    /// Optional 'push' argument. Only works if the 'commit' argument is passed.
    #[arg(short, long)]
    push: bool,

    /// Optional 'push args' argument. Only works if the 'commit' and 'push' arguments are passed.
    #[arg(short)]
    args: Option<Vec<String>>,

    /// Optional 'generate' argument.
    /// Directly generate the `commit_message.md` file.
    #[arg(short, long)]
    generate: bool,

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
/// * `verbose` - Verbose the operation
///
/// ## Returns
/// * `()` - Nothing
fn prepare_commit_msg(
    path: &Path,
    verbose: bool,
) {
    // Get the location of the file passed by 'path'
    // ex: path = /home/user/project/src/main.rs
    // get the location of the file: /home/user/project/src/
    let folder_path = path.parent().unwrap();

    // Get the path to the commit message file
    let gitignore_path = folder_path.join(".gitignore");
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
            let commitignore_items: Vec<String> = git_related::process_gitignore_file(&comitignore_path);
            let gitignore_items: Vec<String> = git_related::process_commitignore_file(&gitignore_path);

            // Check if the file/folder is in the commitignore file or gitignore file
            if
                commitignore_items.contains(&file)
                || gitignore_items.contains(&file)
            {
                // continue means skip the current iteration
                continue;
            }

            // This variable is used to call the 'continue' statement
            // just before the 'writeln!' macro.
            // I can't use the 'continue' statement directly in the for loop
            // because it will skip the next item, not file.
            let mut need_to_skip = false;


            // for each item in the commitignore file and gitignore file,
            // check for file in the folder
            // for example:
            // `data/year_2015/puzzles/` in the commitignore file can
            // exclude `data/year_2015/puzzles/day_01.md` from the commit
            // and in general `data/year_2015/puzzles/*` from the commit
            for item in commitignore_items {
                if check_for_file_in_folder(Path::new(&file), Path::new(&item)) {
                    need_to_skip = true;
                }
            }

            for item in gitignore_items {
                if check_for_file_in_folder(Path::new(&file), Path::new(&item)) {
                    need_to_skip = true;
                }
            }

            if need_to_skip {
                // Skip the current file so the file is not added to the commit message
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

    if verbose {
        // Print a message
        println!(
            "{} {} ✅ ",
            COMMIT_MESSAGE_FILE,
            Green.bold().paint("created")
        );
    }
}

///
/// # create_needed_files
/// Creates the needed files.
///
/// ## Arguments
/// * `verbose` - Verbose the operation
///
/// ## Returns
/// * `()` - Nothing
fn create_needed_files(verbose: bool) {
    if verbose {
        println!("Creating the needed files...");
    }

    // Check if the COMMIT_MESSAGE_FILE exists
    if Path::new(COMMIT_MESSAGE_FILE).exists() {
        if verbose {
            // Print a message
            println!(
                "\t`{}` {} ✅ ",
                COMMIT_MESSAGE_FILE,
                Green.bold().paint("already exists")
            );
        }
    } else {
        // Create the file
        std::fs::File::create(COMMIT_MESSAGE_FILE)
            .expect("Something went wrong creating the file");

        if verbose {
            println!(
                "\t`{}` {} ✅ ",
                COMMIT_MESSAGE_FILE,
                Green.bold().paint("created")
            );
        }
    }

    // Same for the commitignore file
    if Path::new(COMMITIGNORE_FILE_PATH).exists() {
        if verbose {
            println!(
                "\t`{}` {} ✅ ",
                COMMITIGNORE_FILE_PATH,
                Green.bold().paint("already exists")
            );
        }
    } else {
        // Create the file
        std::fs::File::create(COMMITIGNORE_FILE_PATH)
            .expect("Something went wrong creating the file");

        if verbose {
            println!(
                "\t`{}` {} ✅ ",
                COMMITIGNORE_FILE_PATH,
                Green.bold().paint("created")
            );
        }
    }
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
    let cli = Cli::parse();

    // Folder caller - the folder from which the program was called
    let caller = std::env::current_dir().unwrap();


    let commit_message_file_path_str = format!("{}/{}", caller.display(), COMMIT_MESSAGE_FILE);
    let commit_message_file_path = Path::new(&commit_message_file_path_str);

    // Check if the '-v' argument is passed
    let verbose: bool = cli.verbose;

    if cli.commit {
        if commit_message_file_path.exists() {
            // Read the file
            let commit_message = utils::read_file(commit_message_file_path);

            if verbose {
                print_commit_message(commit_message.clone());
            }

            // Commit the changes
            commit(commit_message, verbose).expect("Error commiting the changes");


            if cli.push {
                git_related::push(
                    verbose,
                cli.args.clone()
                ).expect("Error pushing the changes");
            }

        } else {
            // Crash the program
            panic!("{} {} ❌ ", COMMIT_MESSAGE_FILE, Red.bold().paint("not found"));
        }
    } else {
        create_needed_files(verbose);

        if cli.generate {
            prepare_commit_msg(commit_message_file_path, verbose);
        }
    }
}
