///
/// # utils.rs
/// Contains the utils functions.
///

// Imports ================================================================================= Imports
use std::path::Path;
// Functions  ===========================================================================  Functions
///
/// # check_for_file_in_folder
/// Checks if a file is in a folder.
///
/// ## Arguments
/// * `file_path` - The path of the file
/// * `folder_path` - The path of the folder
///
/// ## Example
/// ```
/// use std::path::Path;
///
/// let folder_path = Path::new("data/year_2015/puzzles/");
/// let file_path = Path::new("data/year_2015/puzzles/day_01.md");
///
/// let result = check_for_file_in_folder(file_path, folder_path);
///
/// // Assert the result
/// assert_eq!(result, true);
///
/// ## Returns
/// * `bool` - If the file is in the folder or not
pub fn check_for_file_in_folder(file_path: &Path, folder_path: &Path) -> bool {
    // Get the location of the file passed by 'path'
    // ex: path = /home/user/project/src/main.rs
    // get the location of the file: /home/user/project/src/
    let mut file_parent_path = file_path.parent().unwrap();

    // check if the file is in the folder
    while file_parent_path != folder_path {
        // If the file is not in the folder
        if file_parent_path == Path::new("") {
            return false;
        }

        // Get the parent path
        file_parent_path = file_parent_path.parent().unwrap();
    }

    true
}

///
/// # read_file
/// Reads a file from a source folder.
///
/// ## Arguments
/// * `filename` - The name of the file to read
/// * `source` - The source folder
///
/// ## Returns
/// * `String` - The content of the file
pub fn read_file(path: &Path) -> String {
    // Read and return the file
    std::fs::read_to_string(path).expect("Something went wrong reading the file")
}

// Tests ==================================================================================== Tests
#[test]
fn test_check_for_file_in_folder_direct() {
    let folder_path = Path::new("data/year_2015/puzzles/");
    let file_path = Path::new("data/year_2015/puzzles/day_01.md");

    let result = check_for_file_in_folder(file_path, folder_path);

    // Assert the result
    assert_eq!(result, true);
}

#[test]
fn test_check_for_file_in_folder_indirect() {
    // Path for the folder
    let file_path = Path::new("data/year_2015/puzzles/day_01.md");
    let folder_path_1 = Path::new("data/year_2015");
    let folder_path_2 = Path::new("data/year_2015/puzzles/");
    let folder_path_3 = Path::new("data/");
    let folder_path_4 = Path::new("pipi/");


    // Assert the result
    assert_eq!(check_for_file_in_folder(file_path, folder_path_1), true);
    assert_eq!(check_for_file_in_folder(file_path, folder_path_2), true);
    assert_eq!(check_for_file_in_folder(file_path, folder_path_3), true);
    assert_eq!(check_for_file_in_folder(file_path, folder_path_4), false);
}
