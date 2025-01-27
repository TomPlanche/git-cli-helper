///
/// # `git_related.rs`
/// Contains functions related to git.
// Imports ================================================================================= Imports
use crate::utils::read_file;

use ansi_term::Colour::{Green, Red};
use std::io::{Error, ErrorKind, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

// Functions  ===========================================================================  Functions

// GIT FUNCTIONS ===================================================================== GIT FUNCTIONS

/// # `add_with_exclude`
/// Adds the files to the git index.
/// It will exclude the files and folders passed in the 'exclude' argument.
///
/// ## Arguments
/// * `files_to_exclude` - `&Vec<String>` - the 'paths' of the file to exclude.
/// * `verbose` - `bool` - Should be verbose or not
pub fn add_with_exclude(files_to_exclude: &Vec<String>, verbose: bool) -> bool {
    if verbose {
        println!("Adding files...");
    }

    let _ = Command::new("git")
        .arg("add")
        .arg("--all")
        .output()
        .expect("failed to execute process");

    for file in files_to_exclude {
        if verbose {
            println!("  excuding {file}");
        }

        if Path::new(&file).exists() {
            let _ = Command::new("git")
                .arg("restore")
                .arg("--staged")
                .arg(file)
                .output()
                .expect("failed to execute process");
        }
    }

    true
}

///
/// # commit
/// Commits the changes.
///
/// ## Arguments
/// * `message` - `String` - The commit message
/// * `verbose` - `bool` - If the commit should be verbose or not
///
/// ## Returns
/// * `Result<(), String>` - The result of the commit
pub fn commit(message: &str, verbose: bool) -> Result<bool, String> {
    if verbose {
        println!("Commiting...");
    }

    let final_args: Vec<&str> = vec!["commit", "-m", message];

    let command = Command::new("git")
        .args(final_args)
        .output()
        .expect("failed to execute process");

    if command.status.success() {
        println!("{}", Green.bold().paint("Commit successful."));

        Ok(true)
    } else {
        println!("{}", Red.bold().paint("Commit failed."));

        Err("Commit failed.".to_string())
    }
}

///
/// # push
/// Pushes the changes.
///
/// ## Arguments
/// * `args` - `Option<Vec<String>>` - The args to pass to the command
/// * `verbose` - `bool` - If the push should be verbose or not
///
/// ## Returns
/// * `Result<(), String>` - The result of the push
pub fn push(args: Option<Vec<String>>, verbose: bool) -> Result<(), String> {
    if verbose {
        println!("\nPushing...");
    }

    // Final args for the `git push` command
    let mut final_args: Vec<String> = vec!["push".to_string()];
    final_args.extend(args.unwrap_or_default());

    // Command
    let command = Command::new("git")
        .args(final_args)
        .output()
        .expect("failed to execute process");

    if command.status.success() {
        println!("{}", Green.bold().paint("Push successful."));

        Ok(())
    } else {
        println!("{}", Red.bold().paint("Push failed."));

        Err("Push failed.".to_string())
    }
}

///
/// # `stash_and_mabye_pop`
/// Stashes the changes and maybe pop them.
///
/// ## Arguments
/// * `pop` - `bool` - If the stash should be popped.
pub fn stash_and_maybe_pop(pop: bool) {
    let mut args = vec!["stash".to_string()];
    if pop {
        args.push("pop".to_string());
    } else {
        args.push("-u".to_string());
    }

    let _ = Command::new("git")
        .args(args)
        .output()
        .expect("failed to execute process");
}

///
/// # `switch_branch`
/// Switches the branch.
///
/// ## Arguments
/// * `branch` - `String` - The branch to switch to
pub fn switch_branch(branch: String) {
    let command = Command::new("git")
        .arg("switch")
        .arg(branch)
        .output()
        .expect("failed to execute process");

    if !command.status.success() {
        println!("{}", Red.bold().paint("Failed to switch branch."));
    }
}

/// GETTERS  ==============================================================================  GETTERS
/// # format_branch_name
/// Formats the branch name.
/// If the branch name contains a `COMMIT_TYPES` it will be removed.
///
/// ## Arguments
/// * `commit_types` - `&[&str; 4]` - The commit types
/// * `branch` - `String` - The branch name
///
/// ## Example
/// ```rust
/// let branch = "feat/branch-name";
/// let commit_types = ["feat", "fix", "chore", "docs"];
///
/// let formatted_branch = format_branch_name(&commit_types, branch);
///
/// assert_eq!(formatted_branch, "branch-name");
/// ```
///
/// ## Returns
/// * `String` - The formatted branch name
pub fn format_branch_name(commit_types: &[&str; 4], branch: &str) -> String {
    let mut formatted_branch = branch.to_owned();

    for commit_type in commit_types {
        if formatted_branch.contains(commit_type) {
            // Remove the `/commit_type` from the branch name
            formatted_branch = formatted_branch.replace(&format!("{}/", commit_type), "");
        }
    }

    formatted_branch
}

/// # `get_current_branch`
/// Returns the current git branch.
///
/// ## Returns
/// * `String` - The current git branch
#[allow(dead_code)]
pub fn get_current_branch() -> String {
    // Get the current branch
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("HEAD")
        .output()
        .expect("failed to execute process");

    // Convert the output to a string
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

///
/// # `get_branches_list`
/// Returns the list of git branches of the repository.
///
/// ## Returns
/// * `Vec<String>` - The list of git branches
pub fn get_branches_list() -> Vec<String> {
    // Get the list of branches
    let output = Command::new("git")
        .arg("branch")
        .output()
        .expect("failed to execute process");

    // Convert the output to a string
    let output = String::from_utf8_lossy(&output.stdout);

    output
        .lines()
        .map(|x| {
            x.trim_start_matches("* ")
                .trim_start_matches("  ")
                .to_string()
        })
        .collect()
}

///
/// # `get_current_commit_nb`
/// Returns the number of commits.
///
/// ## Arguments
/// * `branch` - The branch to check - optional - (default: HEAD)
///
/// ## Returns
/// * `u16` - The number of commits
pub fn get_current_commit_nb() -> u16 {
    let output = Command::new("git")
        .arg("rev-list")
        .arg("--count")
        .arg("HEAD")
        .output()
        .expect("failed to execute process");

    String::from_utf8_lossy(&output.stdout)
        .trim()
        .parse::<u16>()
        .unwrap_or(0)
}

// PROCESSING FUNCTIONS ====================================================== PROCESSING FUNCTIONS
///
/// # `process_git_status`
/// Processes the git status.
/// It will parse the git status in order to prepare the git commit message.
///
/// ## Arguments
/// * `message` - The git status
///
/// ## Returns
/// * `Vec<String>` - The modified / added files
pub fn process_git_status(message: &str) -> Vec<String> {
    // Regex to match the modified files and the added files
    let regex_rule = regex::Regex::new(r"^[MTARCU][A-Z\?\! ]\s(.*)$").unwrap();

    message
        .lines()
        .filter_map(|line| {
            if regex_rule.is_match(line) {
                Some(
                    regex_rule
                        .captures(line)
                        .unwrap()
                        .get(1)
                        .unwrap()
                        .as_str()
                        .to_string(),
                )
            } else {
                None
            }
        })
        .collect()
}

///
/// # `process_deleted_files`
/// Processes the deleted files.
///
/// ## Arguments
/// * `message` - The git status
///
/// ## Returns
/// * `Vec<String>` - The deleted files
pub fn process_deteted_files(message: &str) -> Vec<String> {
    // Regex to match the deleted files
    let regex_rule =
        regex::Regex::new(r"^(?:|(?:\sD)|(?:[A-Z]D))\s{1,}([A-Za-z0-9\/_\-\.]*)$").unwrap();

    // Create a vector to store the deleted files while parsing the git status message
    let mut deleted_files: Vec<String> = Vec::new();

    for line in message.lines() {
        if regex_rule.is_match(line) {
            let file_name = regex_rule.captures(line).unwrap().get(1).unwrap().as_str();

            deleted_files.push(file_name.to_string());
        }
    }

    deleted_files
}

///
/// # `process_gitignore_file`
/// Processes the gitignore file.
///
/// ## Arguments
/// * `path` - The path to the gitignore file
///
/// ## Returns
/// * `Vec<String>` - The files and folders to ignore
pub fn process_gitignore_file(path: &Path) -> Vec<String> {
    // look for the gitignore file
    if !path.exists() {
        return Vec::new();
    }

    let git_ignore_file = &read_file(path);

    // The gitignore file stores the files and folders to ignore
    // Each line is a file or folder to ignore
    // The '#' character is used to comment a line

    // Regex to match the files and folders to ignore
    let regex_rule = regex::Regex::new(r"^([^#]\S*)$").unwrap();

    git_ignore_file
        .lines()
        .filter_map(|line| {
            if regex_rule.is_match(line) {
                Some(
                    regex_rule
                        .captures(line)
                        .unwrap()
                        .get(1)
                        .unwrap()
                        .as_str()
                        .to_string(),
                )
            } else {
                None
            }
        })
        .collect()
}

///
/// # `read_git_status`
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
    if command.status.success() {
        // Convert the output to a string
        let output = String::from_utf8_lossy(&command.stdout);

        output.to_string()
    } else {
        // Print an error message
        println!("{}", Red.bold().paint("Failed to read git status."));

        String::new()
    }
}

// Other functions ===============================================================  Other functions
/// # `find_project_root`
/// Finds the project root.
/// May break if the project contains submodules.
///
/// ## Arguments
/// * `caller_path` - The path to the caller.
///
/// ## Returns
/// * `Result<PathBuf, Error>` - The path to the project root, or an error if not found.
pub fn find_git_project_root(caller_path: &Path) -> Result<PathBuf, Error> {
    // Get the path to the caller
    let mut path = caller_path.to_path_buf();

    // While the path is not the root
    while let Some(parent) = path.parent() {
        // If the path contains the .git folder
        if path.join(".git").exists() {
            // Return the path
            return Ok(path);
        }

        // Go up one level
        path = parent.to_path_buf();
    }

    // If we've reached the root '/' without finding a .git folder
    Err(Error::new(
        ErrorKind::NotFound,
        "Git project root not found",
    ))
}

/// # `get_status_files`
/// Returns a list of all files that appear in git status
/// (modified, untracked, staged - but not deleted)
///
/// ## Returns
/// * `Vec<String>` - List of files from git status
pub fn get_status_files() -> Vec<String> {
    let status = read_git_status();

    // Regex to match any file in git status except deleted files
    // Matches patterns like:
    // MM file.txt
    // M  file.txt
    //  M file.txt
    // ?? file.txt
    // But not:
    //  D file.txt
    // AD file.txt
    let regex_rule = regex::Regex::new(r"^[MARCU? ][MARCU? ]\s(.*)$").unwrap();

    // Use a HashSet to avoid duplicates
    use std::collections::HashSet;
    let files: HashSet<String> = status
        .lines()
        .filter_map(|line| {
            // Skip if it's a deleted file
            if line.contains(" D") || line.contains("D ") {
                return None;
            }

            if regex_rule.is_match(line) {
                Some(
                    regex_rule
                        .captures(line)
                        .unwrap()
                        .get(1)
                        .unwrap()
                        .as_str()
                        .to_string(),
                )
            } else {
                println!("Error: unexpected line in git status: {}", line);
                None
            }
        })
        .collect();

    // Convert HashSet back to Vec
    files.into_iter().collect()
}

/// # `add_to_git_exclude`
/// Add paths to the `.git/info/exclude` file.
///
/// ## Arguments
/// * `project_root` - The path to the project root.
/// * `paths` - List of paths to add to the exclude file.
///
/// ## Returns * `Result<(), std::io::Error>` - Result of the operation.
pub fn add_to_git_exclude(project_root: &Path, paths: &[&str]) -> std::io::Result<()> {
    let exclude_file = project_root.join(".git").join("info").join("exclude");
    // Create parent directories if they don't exist
    if let Some(parent) = exclude_file.parent() {
        std::fs::create_dir_all(parent)?;
    }

    // Read existing content to avoid duplicates
    let mut content = String::new();
    if exclude_file.exists() {
        content = std::fs::read_to_string(&exclude_file)?;
    }

    // Open file in append mode
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(exclude_file)?;

    // Add each path if it's not already there
    for path in paths {
        if !content.contains(path) {
            writeln!(file, "{}", path)?;
        }
    }

    Ok(())
}
// Tests ==================================================================================== Tests
#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::{
        add_with_exclude, find_git_project_root, format_branch_name, get_branches_list,
        get_current_branch, get_current_commit_nb, process_deteted_files, process_git_status,
        process_gitignore_file,
    };

    #[test]
    fn test_format_branch_name() {
        const COMMIT_TYPES: [&str; 4] = ["chore", "feat", "fix", "test"];

        let branches: [String; 4] = [
            "chore/branch_name".to_string(),
            "feat/branch_name".to_string(),
            "fix/branch_name".to_string(),
            "test/branch_name".to_string(),
        ];

        for branch in branches.iter() {
            assert_eq!(
                format_branch_name(&COMMIT_TYPES, branch),
                "branch_name".to_string()
            );
        }
    }

    #[test]
    fn test_get_current_branch() {
        assert_eq!(get_current_branch(), "master");
    }

    #[test]
    fn test_get_current_commit_nb() {
        assert_eq!(get_current_commit_nb(), 57)
    }

    #[test]
    fn test_process_git_status() {
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
    fn test_process_deteted_files() {
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

        assert_eq!(
            deleted_files,
            vec!["src/git_related.rs", "src/utils.rs", "src/blo.rs"]
        );
    }

    #[test]
    fn test_add_with_exclude() {
        let exclude: Vec<String> = vec!["README.md".to_string(), "src/git_related.rs".to_string()];

        assert_eq!(add_with_exclude(&exclude, true), true);
    }

    #[test]
    fn test_process_gitignore_file() {
        let gitignore_test_file = Path::new("tests/.gitignore");

        let ignored_files = process_gitignore_file(gitignore_test_file);

        assert_eq!(ignored_files.len() == 9, true);
    }

    #[test]
    fn test_get_branches_list() {
        let branches = get_branches_list();

        assert_eq!(branches.len() == 1, true);
        assert_eq!(branches[0] == "master", true)
    }

    #[test]
    fn test_find_git_project_root() {
        let path = Path::new(".");

        let project_root = find_git_project_root(path);

        assert_eq!(project_root.is_ok(), true);
    }
}
