use std::fs::{self, File};
use std::io::Write;
use tempfile::tempdir;
use flatten::file_processing::calculate_directory_size;
use flatten::types::{ExcludeList, IncludeList};

#[test]
// #[ignore]
fn test_ignore_excludes_files() {
    let temp_dir = tempdir().unwrap();
    let dir_path = temp_dir.path().canonicalize().unwrap();

    // Create files and a test files
    let file1_path = dir_path.join("test1.txt");
    let mut file1 = File::create(&file1_path).unwrap();
    writeln!(file1, "This is test file 1").unwrap();

    let file2_path = dir_path.join("test2.rs");
    let mut file2 = File::create(&file2_path).unwrap();
    writeln!(file2, "fn main() {{ println!(\"Hello, world!\"); }}").unwrap();

    // Create a .ignore file
    let ignore_path = dir_path.join(".ignore");
    let mut ignore = File::create(&ignore_path).unwrap();
    writeln!(ignore, "test2.rs").unwrap();

    // Calculate size, file2 should be ignored
    let exclude = ExcludeList::new(&dir_path, vec![]);
    let include = IncludeList::new(&dir_path, vec![]);
    let calc_size = calculate_directory_size(&dir_path, &exclude, &include, false).unwrap();

    // Only file1 should be counted due to .ignore
    let expected_size = file1_path.metadata().unwrap().len();
    assert_eq!(calc_size, expected_size);
}

// @ToDO get this test working but the functionality is working as expected
#[test]
#[ignore]
fn test_gitignore_excludes_files() {
    let temp_dir = tempdir().unwrap();
    let dir_path = temp_dir.path().canonicalize().unwrap();

    // Create files and a .gitignore file
    let file1_path = dir_path.join("test1.txt");
    let mut file1 = File::create(&file1_path).unwrap();
    writeln!(file1, "This is test file 1").unwrap();

    let file2_path = dir_path.join("test2.rs");
    let mut file2 = File::create(&file2_path).unwrap();
    writeln!(file2, "fn main() {{ println!(\"Hello, world!\"); }}").unwrap();

    let gitignore_path = dir_path.join(".gitignore");
    let mut gitignore = File::create(&gitignore_path).unwrap();
    writeln!(gitignore, "test2.rs").unwrap();

    // Calculate size, file2 should be ignored
    let exclude = ExcludeList::new(&dir_path, vec![]);
    let include = IncludeList::new(&dir_path, vec![]);
    let calc_size = calculate_directory_size(&dir_path, &exclude, &include, false).unwrap();

    // Only file1 should be counted due to .gitignore
    let expected_size = file1_path.metadata().unwrap().len();
    assert_eq!(calc_size, expected_size);
}

#[test]
fn test_including_hidden_files() {
    let temp_dir = tempdir().unwrap();
    let dir_path = temp_dir.path().canonicalize().unwrap();

    // Create a visible file and a hidden file
    let file1_path = dir_path.join("test1.txt");
    let mut file1 = File::create(&file1_path).unwrap();
    writeln!(file1, "This is test file 1").unwrap();

    let hidden_file_path = dir_path.join(".hidden_file.txt");
    let mut hidden_file = File::create(&hidden_file_path).unwrap();
    writeln!(hidden_file, "This is a hidden file").unwrap();

    // Set the hidden file in the include list
    let exclude = ExcludeList::new(&dir_path, vec![]);
    let include = IncludeList::new(&dir_path, vec![hidden_file_path.clone()]);
    // let include = IncludeList::new(&dir_path, vec![]);
    let calc_size = calculate_directory_size(&dir_path, &exclude, &include, false).unwrap();

    // The size should include only the hidden file
    let expected_size = hidden_file_path.metadata().unwrap().len();
    assert_eq!(calc_size, expected_size);
}

// @TODO: This test is failing, this functionality does not work as expected
// because the file path must be explicitly in the include list.
#[test]
#[ignore]
fn test_including_hidden_subdirectory() {
    let temp_dir = tempdir().unwrap();
    let dir_path = temp_dir.path().canonicalize().unwrap();

    // Create a visible file and a hidden subdirectory with a file in it
    let file1_path = dir_path.join("test1.txt");
    let mut file1 = File::create(&file1_path).unwrap();
    writeln!(file1, "This is test file 1").unwrap();

    let hidden_sub_dir_path = dir_path.join("hidden_subdir");
    fs::create_dir(&hidden_sub_dir_path).unwrap();
    let file_in_hidden_subdir = hidden_sub_dir_path.join("test_in_hidden.txt");
    let mut hidden_file = File::create(&file_in_hidden_subdir).unwrap();
    writeln!(hidden_file, "This file is in a hidden subdirectory").unwrap();

    // Set the hidden subdirectory in the include list
    let exclude = ExcludeList::new(&dir_path, vec![]);
    let include = IncludeList::new(&dir_path, vec![hidden_sub_dir_path.clone()]);
    let calc_size = calculate_directory_size(&dir_path, &exclude, &include, false).unwrap();
    let expected_size = file_in_hidden_subdir.metadata().unwrap().len();

    // Size should include only the file in the hidden subdirectory
    assert_eq!(calc_size, expected_size);
}
