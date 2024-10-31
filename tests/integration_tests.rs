use std::fs::{self, File};
use std::io::Write;
use std::process::Command;
use tempfile::tempdir;

#[test]
fn basic_flattening_test() {
    // Setup temporary directory with sample files
    let temp_dir = tempdir().unwrap();
    let dir_path = temp_dir.path();

    let file1_path = dir_path.join("test1.txt");
    let mut file1 = File::create(&file1_path).unwrap();
    writeln!(file1, "This is test file 1").unwrap();

    let file2_path = dir_path.join("test2.rs");
    let mut file2 = File::create(&file2_path).unwrap();
    writeln!(file2, "fn main() {{ println!(\"Hello, world!\"); }}").unwrap();

    // Define output file path
    let output_file = dir_path.join("output.txt");

    // Run flatten command without --directory
    let status = Command::new("cargo")
        .args(&["run", "--"])
        .arg(dir_path)       // Specify directory as positional argument
        .arg("--output")
        .arg(&output_file)
        .status()
        .expect("Failed to execute flatten");

    assert!(status.success());

    // Read and validate output
    let output_content = fs::read_to_string(&output_file).expect("Failed to read output file");
    assert!(output_content.contains("## test1.txt"));
    assert!(output_content.contains("This is test file 1"));
    assert!(output_content.contains("## test2.rs"));
    assert!(output_content.contains("fn main() { println!(\"Hello, world!\"); }"));
}

#[test]
fn include_exclude_combination_test() {
    // Setup temporary directory with sample files
    let temp_dir = tempdir().unwrap();
    let dir_path = temp_dir.path();

    let include_file = dir_path.join("include_this.txt");
    let mut file1 = File::create(&include_file).unwrap();
    writeln!(file1, "This file should be included").unwrap();

    let exclude_file = dir_path.join("exclude_this.txt");
    let mut file2 = File::create(&exclude_file).unwrap();
    writeln!(file2, "This file should be excluded").unwrap();

    let output_file = dir_path.join("output.txt");

    // Run flatten with include and exclude options, directory as positional argument
    let status = Command::new("cargo")
        .args(&["run", "--"])
        .arg(dir_path)
        .arg("--output")
        .arg(&output_file)
        .arg("--include")
        .arg(&include_file)
        .arg("--exclude")
        .arg(&exclude_file)
        .status()
        .expect("Failed to execute flatten");

    assert!(status.success());

    let output_content = fs::read_to_string(&output_file).expect("Failed to read output file");
    assert!(output_content.contains("## include_this.txt"));
    assert!(!output_content.contains("exclude_this.txt"));
}

#[test]
fn hidden_files_test() {
    // Setup temporary directory with hidden and non-hidden files
    let temp_dir = tempdir().unwrap();
    let dir_path = temp_dir.path();

    let hidden_file = dir_path.join(".hidden_file.txt");
    let mut file1 = File::create(&hidden_file).unwrap();
    writeln!(file1, "This is a hidden file").unwrap();

    let visible_file = dir_path.join("visible_file.txt");
    let mut file2 = File::create(&visible_file).unwrap();
    writeln!(file2, "This is a visible file").unwrap();

    let output_file = dir_path.join("output.txt");

    // Run flatten with hidden files allowed, directory as positional argument
    let status = Command::new("cargo")
        .args(&["run", "--"])
        .arg(dir_path)
        .arg("--output")
        .arg(&output_file)
        .arg("--allow-hidden")
        .status()
        .expect("Failed to execute flatten");

    assert!(status.success());

    let output_content = fs::read_to_string(&output_file).expect("Failed to read output file");
    assert!(output_content.contains(".hidden_file.txt"));
    assert!(output_content.contains("This is a hidden file"));
    assert!(output_content.contains("visible_file.txt"));
    assert!(output_content.contains("This is a visible file"));
}

#[test]
fn non_utf8_file_test() {
    // Setup temporary directory with a non-UTF-8 file
    let temp_dir = tempdir().unwrap();
    let dir_path = temp_dir.path();

    let non_utf8_file = dir_path.join("non_utf8_file.bin");
    let mut file = File::create(&non_utf8_file).unwrap();
    file.write_all(b"\xFF\xFE\xFD").expect("Failed to write non-UTF-8 data");

    let output_file = dir_path.join("output.txt");

    // Run flatten, directory as positional argument
    let status = Command::new("cargo")
        .args(&["run", "--"])
        .arg(dir_path)
        .arg("--output")
        .arg(&output_file)
        .status()
        .expect("Failed to execute flatten");

    assert!(status.success());

    // Verify non-UTF-8 file handling in output
    let output_content = fs::read_to_string(&output_file).expect("Failed to read output file");
    assert!(output_content.contains("non_utf8_file.bin"));
    assert!(output_content.contains("<non-UTF-8 data>"));
}
