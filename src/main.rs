///
/// # main.rs
/// Main file for the project.
///
/// ## Arguments
/// * commit - Commit the changes
/// * push - Push the changes
///
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

use crate::utils::check_for_file_in_folder;
use ansi_term::Colour::{Green, Red};
use clap::{Parser, Subcommand};
use dialoguer::{Confirm, Select};
use git_related::{
    add_with_exclude, commit, get_branches_list, get_current_commit_nb, process_deteted_files,
    process_git_status, process_gitignore_file, push, read_git_status, stash_and_maybe_pop,
    switch_branch,
};

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
    /// Commands
    #[command(subcommand)]
    command: Commands,

    /// Verbose
    /// Optional 'verbose' argument. Only works if a subcommand is passed.
    /// If passed, it will print more information about the operation.
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Subcommand)]
#[command(
    about = "Creates all the folders needed for the Advent of Code challenges of the given year."
)]
enum Commands {
    /// Commit subcommand
    /// Directly commit the file with the text in `commit_message.md'.
    #[command(short_flag = 'c')]
    Commit {
        /// Optional 'push' argument. Only works if the 'commit' argument is passed.
        #[arg(short, long)]
        push: bool,

        /// Optional 'push args' argument. Only works if the 'commit' and 'push' arguments are passed.
        #[arg(short)]
        args: Option<Vec<String>>,
    },

    /// Generate subcommand
    /// Directly generate the `commit_message.md` file.
    #[command(short_flag = 'g')]
    Generate,

    /// Push subcommand
    /// Push the changes
    #[command(short_flag = 'p')]
    Push {
        /// Optional 'push args' argument. Only works if the 'commit' and 'push' arguments are passed.
        #[arg(short)]
        args: Option<Vec<String>>,
    },

    /// Add and exclude subcommand
    /// Add all files to the git add command and exclude the files passed by the 'exclude' argument.
    #[command(short_flag = 'a')]
    AddAndExclude {
        /// Files to exclude from the git add command
        #[arg(short, long)]
        exclude: Vec<String>,
    },

    /// Facilitate switching between branches
    #[command(short_flag = 's')]
    Switch {
        /// Stash before switching ?
        #[arg(short, long)]
        stash: bool,

        /// Apply the stach after switching ?
        #[arg(short, long)]
        apply_stash: bool,
    },
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
fn prepare_commit_msg(path: &Path, verbose: bool) {
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
    let git_status: &str = &read_git_status();
    let modified_files: Vec<String> = process_git_status(git_status);
    let deleted_files: Vec<String> = process_deteted_files(git_status);

    // The commit message file
    let mut commit_file = std::fs::OpenOptions::new()
        .append(true) // Append to the file
        .open(path)
        .unwrap();

    let commit_number: u16 = get_current_commit_nb() + 1;

    if let Err(e) = writeln!(commit_file, "[{}]\n\n", commit_number) {
        eprintln!("Couldn't write to file: {}", e);
    }

    // For each modified file
    for file in modified_files {
        // If the file is not a file in the commitignore file
        // or is not in a folder in the commitignore file
        if comitignore_path.exists() {
            let mut items_to_ignore: Vec<String> = process_gitignore_file(&gitignore_path);
            items_to_ignore.append(&mut process_gitignore_file(&comitignore_path));

            // Check if the file/folder is in the commitignore file or gitignore file
            if items_to_ignore.contains(&file) {
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
            for item in items_to_ignore {
                if check_for_file_in_folder(Path::new(&file), Path::new(&item)) {
                    need_to_skip = true;
                }
            }

            if need_to_skip {
                // Skip the current file so the file is not added to the commit message
                continue;
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

    // For each deleted file
    for file in deleted_files {
        if let Err(e) = writeln!(commit_file, "- `{}`: deleted\n", file) {
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
        std::fs::File::create(COMMIT_MESSAGE_FILE).expect("Something went wrong creating the file");

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

    let verbose = cli.verbose;

    match &cli.command {
        Commands::AddAndExclude { exclude } => {
            println!("{:?}", exclude);

            let successful_add =
                add_with_exclude(exclude, verbose).expect("Error adding the files");

            if successful_add {
                println!("{} ✅ ", Green.bold().paint("Added the files"));
            } else {
                println!("{} ❌ ", Red.bold().paint("Error adding the files"));
            }
        }
        Commands::Commit { push, args } => {
            if commit_message_file_path.exists() {
                // Read the file
                let commit_message = utils::read_file(commit_message_file_path);

                if verbose {
                    print_commit_message(commit_message.clone());
                }

                // Commit the changes
                let succesfull_commit =
                    commit(commit_message, verbose).expect("Error commiting the changes");

                if *push && succesfull_commit {
                    git_related::push(args.clone(), verbose.clone())
                        .expect("Error pushing the changes");
                }
            } else {
                // Crash the program
                panic!(
                    "{} {} ❌ ",
                    COMMIT_MESSAGE_FILE,
                    Red.bold().paint("not found.")
                );
            }
        }

        Commands::Generate => {
            create_needed_files(verbose);

            prepare_commit_msg(commit_message_file_path, verbose);
        }

        Commands::Push { args } => {
            push(args.clone(), verbose).expect("Error pushing the changes");
        }

        Commands::Switch { stash, apply_stash } => {
            if *stash {
                println!("Stashing changes...");
                stash_and_maybe_pop(false);
            }

            let branches: Vec<String> = get_branches_list();

            let chosen_branch = &branches[Select::new()
                .with_prompt("Choose a branch")
                .default(0)
                .items(&branches)
                .interact()
                .unwrap()];

            if Confirm::new()
                .with_prompt(&format!("Switch to branch: {} ?", chosen_branch))
                .default(true)
                .interact()
                .unwrap()
            {
                switch_branch(chosen_branch.to_string());

                if *apply_stash {
                    stash_and_maybe_pop(true);
                }
            } else {
                println!("Aborted");
            }
        }
    }
}
