///
/// # git_related
/// Contains functions related to git.
// Imports ================================================================================= Imports
use crate::utils::read_file;

use ansi_term::Colour::{Green, Red};
use std::path::Path;
use std::process::Command;

// Functions  ===========================================================================  Functions
/// # add_with_exclude
/// Adds the files to the git index.
/// It will exclude the files and folders passed in the 'exclude' argument.
///
/// ## Arguments
/// * `exclude` - The files and folders to exclude
/// * `verbose` - If the add should be verbose or not
pub fn add_with_exclude(exclude: &Vec<String>, verbose: bool) -> Result<bool, String> {
    if verbose {
        println!("Adding files...");
    }

    let _ = Command::new("git")
        .arg("add")
        .arg("--all")
        .output()
        .expect("failed to execute process");

    for document in exclude {
        // check if the document exists
        if Path::new(&document).exists() {
            // rm command
            let _ = Command::new("git")
                .arg("rm")
                .arg("-r")
                .arg("--cached")
                .arg(document)
                .output()
                .expect("failed to execute process");
        }
    }

    Ok(true)
}

///
/// # commit
/// Commits the changes.
///
/// ## Arguments
/// * `message` - The commit message
/// * `verbose` - If the commit should be verbose or not
///
/// ## Returns
/// * `Result<(), String>` - The result of the commit
pub fn commit(message: String, verbose: bool) -> Result<bool, String> {
    if verbose {
        println!("Commiting...");
    }

    let final_args: Vec<&str> = vec!["commit", "-m", message.as_str()];

    // Command
    let command = Command::new("git")
        .args(final_args)
        .output()
        .expect("failed to execute process");

    // If the command was successful
    if command.status.success() {
        // Print a success message
        println!("{}", Green.bold().paint("Commit successful."));

        Ok(true)
    } else {
        // Print an error message
        println!("{}", Red.bold().paint("Commit failed."));

        Err("Commit failed.".to_string())
    }
}

///
/// # push
/// Pushes the changes.
///
/// ## Arguments
/// * `args` - The args to pass to the command
/// * `verbose` - If the push should be verbose or not
///
/// ## Returns
/// * `Result<(), String>` - The result of the push
pub fn push(args: Option<Vec<String>>, verbose: bool) -> Result<(), String> {
    if verbose {
        println!("\nPushing...");
    }

    let mut final_args: Vec<String> = vec!["push".to_string()];

    final_args.extend(args.unwrap_or_default());

    // Command
    let command = Command::new("git")
        .args(final_args)
        .output()
        .expect("failed to execute process");

    // If the command was successful
    if command.status.success() {
        // Print a success message
        println!("{}", Green.bold().paint("Push successful."));

        Ok(())
    } else {
        // Print an error message
        println!("{}", Red.bold().paint("Push failed."));

        Err("Push failed.".to_string())
    }
}

///
/// # get_current_branch
/// Returns the current git branch.
///
/// ## Arguments
/// * `()` - Nothing
///
/// ## Returns
/// * `String` - The current git branch
pub fn get_current_branch() -> String {
    // Get the current branch
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("HEAD")
        .output()
        .expect("failed to execute process");

    // Convert the output to a string
    let output = String::from_utf8_lossy(&output.stdout);

    // Remove the '\n' at the end of the string
    let output = output.trim();

    // Return the current branch
    output.to_string()
}

///
/// # get_current_commit_nb
/// Returns the number of commits.
///
/// ## Arguments
/// * `branch` - The branch to check - optional - (default: HEAD)
///
/// ## Returns
/// * `u16` - The number of commits
pub fn get_current_commit_nb(branch: Option<&str>) -> u16 {
    let output = Command::new("git")
        .arg("rev-list")
        .arg("--count")
        .arg(branch.unwrap_or(get_current_branch().as_str()))
        .output()
        .expect("failed to execute process");

    return String::from_utf8_lossy(&output.stdout)
        .trim()
        .parse::<u16>()
        .unwrap_or(0);
}

///
/// # process_git_status
/// Processes the git status.
/// It will parse the git status in order to prepare the git commit message.
///
/// ## Arguments
/// * `message` - The git status
///
/// ## Returns
/// * `Vec<String>` - The modified / added files
pub fn process_git_status(message: &str) -> Vec<String> {
    // Modified files are indicated by a 'M' at the beginning of the line
    // Added files are indicated by a 'A' at the beginning of the line

    // Regex to match the modified files and the added files
    let regex_rule = regex::Regex::new(r"^[MTARCU][A-Z\?\! ]\s(.*)$").unwrap();

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
    modified_files
}

///
/// # process_deleted_files
/// Processes the deleted files.
///
/// ## Arguments
/// * `message` - The git status
///
/// ## Returns
/// * `Vec<String>` - The deleted files
pub fn process_deteted_files(message: &str) -> Vec<String> {
    // Deleted files are indicated by a 'D' at the beginning of the line

    // Regex to match the deleted files
    let regex_rule =
        regex::Regex::new(r"^(?:(?:D+)|(?:\sD)|(?:[A-Z]D))\s{1,}([A-Za-z0-9\/_\.]*)$").unwrap();

    // Create a vector to store the deleted files while parsing the git status message
    let mut deleted_files: Vec<String> = Vec::new();

    // For each line in the git status message
    for line in message.lines() {
        // If the line matches the regex
        if regex_rule.is_match(line) {
            // Get the file name
            let file_name = regex_rule.captures(line).unwrap().get(1).unwrap().as_str();

            // Add the file name to the vector
            deleted_files.push(file_name.to_string());
        }
    }

    // Return the vector
    deleted_files
}

///
/// # process_gitignore_file
/// Processes the gitignore file.
///
/// ## Arguments
/// * `path` - The path to the gitignore file
pub fn process_gitignore_file(path: &Path) -> Vec<String> {
    // Read the gitignore file
    let gitignore_file = &read_file(path);

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

    files_to_ignore
}

///
/// # process_commitignore_file
/// Processes the commitignore file.
/// It will parse the commitignore file in order to prepare the git commit message.
///
/// ## Arguments
/// * `path` - The path to the commitignore file
///
/// ## Returns
/// * `Vec<String>` - The files and folders to ignore
pub fn process_commitignore_file(path: &Path) -> Vec<String> {
    // Read the commitignore file
    let commitignore_file = &read_file(path);

    // The commitignore file stores the files and folders to ignore
    // Each line is a file or folder to ignore
    // The '#' character is used to comment a line

    // Regex to match the files and folders to ignore
    let regex_rule = regex::Regex::new(r"^[^#](.*)$").unwrap();

    // Create a vector to store the files and folders to ignore while parsing the commitignore file
    let mut files_to_ignore: Vec<String> = Vec::new();

    // For each line in the commitignore file
    for line in commitignore_file.lines() {
        // If the line matches the regex
        if regex_rule.is_match(line) {
            // Add the file or folder name to the vector
            files_to_ignore.push(line.to_string());
        }
    }

    files_to_ignore
}

///
/// # read_git_status
/// Reads the git status.
///
/// ## Returns
/// * `String` - The git status
pub fn read_git_status() -> String {
    // Command
    let command = Command::new("git")
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
        println!("{}", Red.bold().paint("Failed to read git status."));

        String::from("")
    };
}

// Tests ==================================================================================== Tests
#[cfg(test)]
mod tests {
    use super::{add_with_exclude, get_current_branch, process_deteted_files, process_git_status};

    #[test]
    fn test_get_current_branch() {
        assert_eq!(get_current_branch(), "master");
    }

    #[test]
    fn test_regex_process_git_status() {
        let lines: Vec<&str> = vec![
            " M src/git_related.rs",
            "M  src/main.rs",
            "AM src/utils.rs",
            "?? src/README.md",
            "UU src/bla.rs",
            "!! src/bli.rs",
            "DD src/blo.rs",
            "R  src/blu.rs",
            "C  src/bly.rs",
            "U  src/pae.rs",
        ];

        let modified_files = process_git_status(lines.join("\n").as_str());

        assert_eq!(
            modified_files,
            vec![
                "src/main.rs",
                "src/utils.rs",
                "src/bla.rs",
                "src/blu.rs",
                "src/bly.rs",
                "src/pae.rs",
            ]
        );
    }

    #[test]
    fn test_regex_process_deteted_files() {
        let lines: Vec<&str> = vec![
            " D src/git_related.rs",
            "D  src/main.rs",
            "AD src/utils.rs",
            "?? src/README.md",
            "UU src/bla.rs",
            "!! src/bli.rs",
            "DD src/blo.rs",
            "R  src/blu.rs",
            "C  src/bly.rs",
            "U  src/pae.rs",
        ];
        let deleted_files = process_deteted_files(lines.join("\n").as_str());

        assert_eq!(deleted_files, vec!["src/main.rs", "src/blo.rs",]);
    }

    #[test]
    fn test_add_with_exclude() {
        let exclude: Vec<String> = vec!["README.md".to_string(), "src/git_related.rs".to_string()];

        assert_eq!(add_with_exclude(&exclude, true).unwrap(), true);
    }
}
