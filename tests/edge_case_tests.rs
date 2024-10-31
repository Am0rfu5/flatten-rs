use std::fs::{File};
use std::io::Write;
use tempfile::tempdir;
use flatten::file_processing::{process_files, calculate_directory_size};
use flatten::types::{ExcludeList, IncludeList};

#[test]
fn test_empty_file_processing() {
    let temp_dir = tempdir().unwrap();
    let dir_path = temp_dir.path().canonicalize().unwrap();

    // Create an empty file
    let empty_file_path = dir_path.join("empty_file.txt");
    File::create(&empty_file_path).unwrap();

    // Output file
    let output_file = dir_path.join("output.txt");

    // Process the directory containing the empty file
    let exclude = ExcludeList::new(&dir_path, vec![]);
    let include = IncludeList::new(&dir_path, vec![]);
    process_files(&dir_path, &output_file, &exclude, &include, true).unwrap();

    // Check output file content
    let output_content = std::fs::read_to_string(output_file).unwrap();
    assert!(output_content.contains("## empty_file.txt"));
    assert!(output_content.contains("```plain text\n\n```"));
}

#[test]
fn test_large_file_processing() {
    let temp_dir = tempdir().unwrap();
    let dir_path = temp_dir.path().canonicalize().unwrap();

    // Create a large file (5 MB)
    let large_file_path = dir_path.join("large_file.txt");
    let mut large_file = File::create(&large_file_path).unwrap();
    for _ in 0..5 * 1024 * 1024 {
        large_file.write_all(b"a").unwrap();
    }

    // Output file
    let output_file = dir_path.join("output.txt");

    // Process the directory containing the large file
    let exclude = ExcludeList::new(&dir_path, vec![]);
    let include = IncludeList::new(&dir_path, vec![]);
    process_files(&dir_path, &output_file, &exclude, &include, true).unwrap();

    // Verify that the output file contains the header for the large file
    let output_content = std::fs::read_to_string(output_file).unwrap();
    assert!(output_content.contains("## large_file.txt"));
}

#[test]
fn test_non_utf8_file_processing() {
    let temp_dir = tempdir().unwrap();
    let dir_path = temp_dir.path().canonicalize().unwrap();

    // Create a non-UTF-8 file
    let non_utf8_file_path = dir_path.join("non_utf8_file.bin");
    let mut non_utf8_file = File::create(&non_utf8_file_path).unwrap();
    non_utf8_file.write_all(b"\xFF\xFE\xFD").expect("Failed to write non-UTF-8 data");

    // Output file
    let output_file = dir_path.join("output.txt");

    // Process the directory containing the non-UTF-8 file
    let exclude = ExcludeList::new(&dir_path, vec![]);
    let include = IncludeList::new(&dir_path, vec![]);
    process_files(&dir_path, &output_file, &exclude, &include, true).unwrap();

    // Verify that the output contains a placeholder for non-UTF-8 data
    let output_content = std::fs::read_to_string(output_file).unwrap();
    assert!(output_content.contains("## non_utf8_file.bin"));
    assert!(output_content.contains("<non-UTF-8 data>"));
}

#[test]
fn test_directory_size_with_large_file() {
    let temp_dir = tempdir().unwrap();
    let dir_path = temp_dir.path().canonicalize().unwrap();

    // Create a large file (1 MB)
    let large_file_path = dir_path.join("large_file.txt");
    let mut large_file = File::create(&large_file_path).unwrap();
    for _ in 0..1024 * 1024 {
        large_file.write_all(b"a").unwrap();
    }

    // Calculate the directory size
    let exclude = ExcludeList::new(&dir_path, vec![]);
    let include = IncludeList::new(&dir_path, vec![]);
    let size = calculate_directory_size(&dir_path, &exclude, &include, true).unwrap();

    // Verify the size matches the large file's size
    assert_eq!(size, large_file_path.metadata().unwrap().len());
}
