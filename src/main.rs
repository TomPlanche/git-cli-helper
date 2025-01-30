///
/// # main.rs
/// Main file for the project.
///
/// Read the [README.md](../README.md) for more information.
// Imports ================================================================================= Imports
#[path = "./utils.rs"]
mod utils;

#[path = "git_related.rs"]
mod git_related;

#[path = "my_theme.rs"]
mod my_theme;

use std::path::{Path, PathBuf};
use std::{fs::File, io::prelude::Write};

use ansi_term::Colour::{Green, Red};
use clap::{Parser, Subcommand};
use dialoguer::{Confirm, Select};
use git_related::{
    add_to_git_exclude, add_with_exclude, commit, find_git_project_root, format_branch_name,
    get_branches_list, get_current_branch, get_current_commit_nb, process_deteted_files,
    process_git_status, process_gitignore_file, push, read_git_status, stash_and_maybe_pop,
    switch_branch,
};
use utils::check_for_file_in_folder;

// Constants  ===========================================================================  Constants
const GITIGNORE_FILE_PATH: &str = ".gitignore";
const COMMIT_MESSAGE_FILE: &str = "commit_message.md";
const COMMITIGNORE_FILE_PATH: &str = ".commitignore";
const COMMIT_TYPES: [&str; 4] = ["chore", "feat", "fix", "test"];

// Args commands

// Cli parser
#[derive(Parser)]
#[command(about = "Custom program that can:\n\
\t- Commit with the current 'commit_message.md' file text.\n\
\t- Generates the 'commit_message.md' file.")]
#[command(author = "Tom P. <tomplanche@icloud.com>")]
#[command(help_template = "{about}\nMade by: {author}\n\nUSAGE:\n{usage}\n\n{all-args}\n")]
#[command(name = "git-commands")]
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
    /// Add and exclude subcommand
    /// Add all files to the git add command and exclude the files passed by the 'exclude' argument.
    #[command(short_flag = 'a')]
    AddAndExclude {
        /// Files to exclude from the git add command
        #[arg(short, long)]
        exclude: Vec<String>,
    },

    /// Commit subcommand
    /// Directly commit the file with the text in `commit_message.md`.
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

    /// List files from git status (for shell completion)
    #[command(short_flag = 'l')]
    ListStatus,
}
// Function(s) =========================================================================== Functions
///
/// # `prepare_commit_msg`
/// Prepares the commit message.
/// It creates the commit message file and empty it if it already exists.
/// It also adds the modified / added files to the commit message file.
///
/// ## Arguments
/// * `path` - `&Path` - The source folder
/// * `commit_types` - `&str` - The commit types
/// * `verbose` - `bool` - Verbose the operation
fn prepare_commit_msg(path: &Path, commit_type: &str, verbose: bool) {
    // Get the location of the file passed by 'path'
    // ex: path = /home/user/project/src/main.rs
    // get the location of the file: /home/user/project/src/
    let folder_path = path.parent().unwrap();

    // Get the path to the commit message file
    let gitignore_path = folder_path.join(GITIGNORE_FILE_PATH);
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
    let branch_name: &str = &format_branch_name(&COMMIT_TYPES, &get_current_branch());

    if let Err(e) = writeln!(
        commit_file,
        "[{commit_number}] ({commit_type} on {branch_name})\n\n"
    ) {
        eprintln!("Couldn't write to file: {e}");
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
        }

        if let Err(e) = writeln!(commit_file, "- `{file}`:\n\n\t\n") {
            eprintln!("Couldn't write to file: {e}");
        }
    }

    // For each deleted file
    for file in deleted_files {
        if let Err(e) = writeln!(commit_file, "- `{file}`: deleted\n") {
            eprintln!("Couldn't write to file: {e}");
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

/// # `create_needed_files`
/// Creates the needed files in the specified project root.
///
/// ## Arguments
/// * `path` - `PathBuf` - The path to the project root
/// * `verbose` - `bool` - Verbose the operation
fn create_needed_files(path: &PathBuf, verbose: bool) {
    if verbose {
        println!("Creating the needed files in {path:?}...");
    }

    let commit_message_path = path.join(COMMIT_MESSAGE_FILE);
    let commitignore_path = path.join(COMMITIGNORE_FILE_PATH);

    // Check if the COMMIT_MESSAGE_FILE exists
    if commit_message_path.exists() {
        if verbose {
            println!(
                "\t`{}` {} ✅ ",
                commit_message_path.display(),
                Green.bold().paint("already exists")
            );
        }
    } else {
        // Create the file
        File::create(&commit_message_path)
            .expect("Something went wrong creating the commit message file");

        if verbose {
            println!(
                "\t`{}` {} ✅ ",
                commit_message_path.display(),
                Green.bold().paint("created")
            );
        }
    }

    // Same for the commitignore file
    if commitignore_path.exists() {
        if verbose {
            println!(
                "\t`{}` {} ✅ ",
                commitignore_path.display(),
                Green.bold().paint("already exists")
            );
        }
    } else {
        // Create the file
        File::create(&commitignore_path)
            .expect("Something went wrong creating the commitignore file");

        if verbose {
            println!(
                "\t`{}` {} ✅ ",
                commitignore_path.display(),
                Green.bold().paint("created")
            );
        }
    }

    // Add files to git exclude
    if let Err(e) = add_to_git_exclude(path, &[COMMIT_MESSAGE_FILE, COMMITIGNORE_FILE_PATH]) {
        eprintln!("Warning: Failed to add files to git exclude: {e}");
    }
}

///
/// # `print_commit_message`
/// Prints the commit message.
///
/// ## Arguments
/// * `commit_message` - The commit message
///
/// ## Returns
/// * `()` - Nothing
fn print_commit_message(commit_message: &str) {
    let delimiter = "------------------------------------------------";
    println!("\nCommit message: \n{delimiter}\n{commit_message}\n{delimiter}");
}
// MAIN ======================================================================================= MAIN
fn main() {
    // Read the passed arguments
    let cli = Cli::parse();

    // Folder caller - the folder from which the program was called
    let caller = std::env::current_dir().unwrap();

    let project_root = find_git_project_root(&caller).unwrap();

    let commit_message_file_path_str =
        format!("{}/{}", project_root.display(), COMMIT_MESSAGE_FILE);
    let commit_message_file_path = Path::new(&commit_message_file_path_str);

    let verbose = cli.verbose;

    match &cli.command {
        Commands::AddAndExclude { exclude } => {
            let (successful_add, successfully_exclude) = add_with_exclude(exclude, verbose);

            println!("Added {successful_add} files to the commit and excluded {successfully_exclude} files.");
        }
        Commands::Commit { push, args } => {
            if commit_message_file_path.exists() {
                // Read the file
                let commit_message = utils::read_file(commit_message_file_path);

                if verbose {
                    print_commit_message(&commit_message.clone());
                }

                // Commit the changes
                let succesfull_commit =
                    commit(&commit_message, verbose).expect("Error commiting the changes");

                if *push && succesfull_commit {
                    git_related::push(args.clone(), verbose).expect("Error pushing the changes");
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
            create_needed_files(&project_root, verbose);

            let commit_type = COMMIT_TYPES[Select::with_theme(&my_theme::ColorfulTheme::default())
                .default(0)
                .items(&COMMIT_TYPES)
                .interact()
                .unwrap()];

            prepare_commit_msg(commit_message_file_path, commit_type, verbose);

            // Open the file in Zed.
            let _ = std::process::Command::new("zed")
                .arg(commit_message_file_path)
                .spawn()
                .expect("Error opening the file in Zed")
                .wait();
        }

        Commands::Push { args } => {
            push(args.clone(), verbose).expect("Error pushing the changes");
        }

        Commands::Switch { stash, apply_stash } => {
            // prevent stash AND apply_stash
            if *stash && *apply_stash {
                eprintln!("❌ You can't use --stash and --apply-stash at the same time.");
                std::process::exit(1);
            }

            if *stash {
                stash_and_maybe_pop(false);
            }

            let branches: Vec<String> = get_branches_list();

            let chosen_branch = &branches[Select::with_theme(&my_theme::ColorfulTheme::default())
                .default(0)
                .items(&branches)
                .interact()
                .unwrap()];

            if Confirm::with_theme(&my_theme::ColorfulTheme::default())
                .with_prompt(format!("Switch to branch: {chosen_branch} ?"))
                .default(true)
                .interact()
                .unwrap()
            {
                switch_branch(chosen_branch.to_string());

                if *apply_stash {
                    stash_and_maybe_pop(true);
                }
            } else {
                println!("Bye !");
            }
        }

        Commands::ListStatus => {
            let files = git_related::get_status_files();
            // Print each file on a new line for fish shell completion
            for file in files {
                println!("{file}");
            }
        }
    }
}
