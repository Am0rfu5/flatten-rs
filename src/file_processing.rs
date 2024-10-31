use std::fs::File;
use std::io::{self, Write, Read};
use std::path::PathBuf;
use syntect::parsing::SyntaxSet;
use crate::types::{ExcludeList, IncludeList};
use ignore::{WalkBuilder, Walk, overrides::OverrideBuilder};

/// Constructs a file walker that recursively traverses a directory with specified
/// include and exclude filters.
///
/// # Arguments
///
/// * `directory` - The base directory to walk.
/// * `exclude` - An `ExcludeList` specifying files/directories to ignore.
/// * `include` - An `IncludeList` specifying files/directories to include.
/// * `allow_hidden` - A boolean indicating whether to include hidden files.
///
/// # Returns
///
/// * `Ok(Walk)` - The configured file walker on success.
/// * `Err(ignore::Error)` - An error if building the walker fails.
fn build_walker(directory: &PathBuf, exclude: &ExcludeList, include: &IncludeList, allow_hidden: bool) -> Result<Walk, ignore::Error> {
    let mut overrides = OverrideBuilder::new(directory);

    // Add include patterns
    for inc in &include.0 {
        let inc_pattern = format!("{}", inc.display());
        overrides.add(&inc_pattern)?;
    }

    // Add exclude patterns
    for exc in &exclude.0 {
        let exc_pattern = format!("{}", exc.display());
        overrides.add(&exc_pattern)?;
    }

    let overrides = overrides.build()?;

    let walker = WalkBuilder::new(directory)
        .overrides(overrides)
        .hidden(!allow_hidden)
        .build();

    Ok(walker)
}

/// Processes all files in a directory and writes their contents into a single output file.
///
/// # Arguments
///
/// * `directory` - The directory containing files to process.
/// * `output_file` - The output file where contents are consolidated.
/// * `exclude` - An `ExcludeList` specifying files/directories to ignore.
/// * `include` - An `IncludeList` specifying files/directories to include.
/// * `allow_hidden` - Indicates whether hidden files are included in processing.
///
/// # Returns
///
/// * `Ok(())` if successful.
/// * `Err(io::Error)` if file processing or writing fails.
pub fn process_files(directory: &PathBuf, output_file: &PathBuf, exclude: &ExcludeList, include: &IncludeList, allow_hidden: bool) -> io::Result<()> {

    let mut output = File::create(output_file)?;
    let ss = SyntaxSet::load_defaults_newlines();
    let walker = build_walker(directory, exclude, include, allow_hidden).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    for result in walker {
        let entry = result.map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        let path = entry.path().canonicalize().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        
        if entry.file_type().map_or(false, |ft| ft.is_file()) && path != output_file.canonicalize().map_err(|e| io::Error::new(io::ErrorKind::Other, e))? {
            let rel_path = path.strip_prefix(directory).unwrap_or(&path);
            let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("txt");
            let syntax = ss.find_syntax_by_extension(extension).unwrap_or_else(|| ss.find_syntax_plain_text());

            writeln!(output, "## {}", rel_path.display())?;
            writeln!(output, "```{}", syntax.name.to_lowercase())?;

            let mut file = File::open(&path)?;
            let mut contents = Vec::new();
            file.read_to_end(&mut contents)?;

            match String::from_utf8(contents) {
                Ok(text) => writeln!(output, "{}", text)?,
                Err(_) => writeln!(output, "<non-UTF-8 data>")?,
            }

            writeln!(output, "```")?;
            writeln!(output)?; // Add an empty line between files
        }
    }

    Ok(())
}


/// Calculates the cumulative size of all files in a directory, excluding or including
/// specific files based on provided filters.
///
/// # Arguments
///
/// * `directory` - The directory containing files to calculate.
/// * `exclude` - An `ExcludeList` specifying files/directories to ignore.
/// * `include` - An `IncludeList` specifying files/directories to include.
/// * `allow_hidden` - A flag to determine if hidden files are counted.
///
/// # Returns
///
/// * `Ok(u64)` - The total size in bytes of all included files.
/// * `Err(io::Error)` - If any file operation fails.
pub fn calculate_directory_size(directory: &PathBuf, exclude: &ExcludeList, include: &IncludeList, allow_hidden: bool) -> io::Result<u64> {

    let walker = build_walker(directory, exclude, include, allow_hidden).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    let mut size = 0;

    for result in walker {
        let entry = result.map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        if entry.file_type().map_or(false, |ft| ft.is_file()) {
            size += entry.metadata().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?.len();
        }
    }

    Ok(size)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_process_files() {
        let temp_dir = tempdir().unwrap();
        let dir_path = temp_dir.path().canonicalize().unwrap();

        // Create sample files
        let file1_path = dir_path.join("test1.txt");
        let mut file1 = File::create(&file1_path).unwrap();
        writeln!(file1, "This is test file 1").unwrap();

        let file2_path = dir_path.join("test2.rs");
        let mut file2 = File::create(&file2_path).unwrap();
        writeln!(file2, "fn main() {{ println!(\"Hello, world!\"); }}").unwrap();

        // Output file
        let output_path = dir_path.join("output.txt");

        // Process files
        let exclude = ExcludeList::new(&dir_path, vec![]);
        let include = IncludeList::new(&dir_path, vec![]);
        process_files(&dir_path, &output_path, &exclude, &include, true).unwrap();

        // Read output file
        let output_content = fs::read_to_string(output_path).unwrap();

        // Assertions
        assert!(output_content.contains("##"));
        assert!(output_content.contains("This is test file 1"));
        assert!(output_content.contains("fn main() { println!(\"Hello, world!\"); }"));
        assert!(output_content.contains("```rust"));
        assert!(output_content.contains("```plain text"));
    }

    #[test]
    fn test_calculate_directory_size() {
        let temp_dir = tempdir().unwrap();
        let dir_path = temp_dir.path().canonicalize().unwrap();

        // Create sample files
        let file1_path = dir_path.join("test1.txt");
        let mut file1 = File::create(&file1_path).unwrap();
        writeln!(file1, "This is test file 1").unwrap();

        let file2_path = dir_path.join("test2.rs");
        let mut file2 = File::create(&file2_path).unwrap();
        writeln!(file2, "fn main() {{ println!(\"Hello, world!\"); }}").unwrap();

        // Calculate size
        let exclude = ExcludeList::new(&dir_path, vec![]);
        let include = IncludeList::new(&dir_path, vec![]);
        let size = calculate_directory_size(&dir_path, &exclude, &include, true).unwrap();

        // Assertions
        assert_eq!(size, file1_path.metadata().unwrap().len() + file2_path.metadata().unwrap().len());
    }

    #[test]
    fn test_calculate_directory_size_with_excludes() {
        let temp_dir = tempdir().unwrap();
        let dir_path = temp_dir.path().canonicalize().unwrap();

        // Create sample files
        let file1_path = dir_path.join("test1.txt");
        let mut file1 = File::create(&file1_path).unwrap();
        writeln!(file1, "This is test file 1").unwrap();

        let file2_path = dir_path.join("test2.rs");
        let mut file2 = File::create(&file2_path).unwrap();
        writeln!(file2, "fn main() {{ println!(\"Hello, world!\"); }}").unwrap();

        // Exclude test1.txt
        let exclude = ExcludeList::new(&dir_path, 
            vec![PathBuf::from("test1.txt")]);
        let include = IncludeList::new(&dir_path, vec![]);
        let size = calculate_directory_size(&dir_path, &exclude, &include, 
            true).unwrap();

        // Assertions
        assert_eq!(size, file2_path.metadata().unwrap().len());
    }

    #[test]
    fn test_calculate_directory_size_with_includes() {
        let temp_dir = tempdir().unwrap();
        let dir_path = temp_dir.path().canonicalize().unwrap();
        let sub_dir_path = temp_dir.path().join("subdir");
    
        // Create an empty exclude and include list
        let exclude = ExcludeList::new(&dir_path, vec![]);
        let include = IncludeList::new(&dir_path, vec![]);
    
        // Create a text file
        let file1_path = dir_path.join("test1.txt");
        let mut file1 = File::create(&file1_path).unwrap();
        writeln!(file1, "This is test file 1").unwrap();
    
        // Calculate the expected size = file1 size
        let expected_size = file1_path.metadata().unwrap().len();
        let calc_size = calculate_directory_size(&dir_path, &exclude, &include, false).unwrap();
    
        assert_eq!(calc_size, expected_size);
    
        // Create a hidden text file
        let hidden_file_path = dir_path.join(".hidden_file.txt");
        let mut hidden_file = File::create(&hidden_file_path).unwrap();
        writeln!(hidden_file, "This is a hidden file").unwrap();
    
        // Recalculate size, it should still only include `test1.txt` since `allow_hidden` is false
        let calc_size = calculate_directory_size(&dir_path, &exclude, &include, false).unwrap();
        assert_eq!(calc_size, expected_size);
    
        // Create a Rust file
        let file2_path = dir_path.join("test2.rs");
        let mut file2 = File::create(&file2_path).unwrap();
        writeln!(file2, "fn main() {{ println!(\"Hello, world!\"); }}").unwrap();
    
        let expected_size = expected_size + file2_path.metadata().unwrap().len();
        let calc_size = calculate_directory_size(&dir_path, &exclude, &include, false).unwrap();
        assert_eq!(calc_size, expected_size);
    
        // Create a file in the subdirectory `subdir`
        fs::create_dir(&sub_dir_path).unwrap();
        let file3_path = sub_dir_path.join("test3.md");
        let mut file3 = File::create(&file3_path).unwrap();
        writeln!(file3, "fn main() {{ println!(\"# MARKDOWN!\"); }}").unwrap();
    
        // Create a hidden subdirectory with a file in it
        let hidden_sub_dir_path = dir_path.join(".hidden_subdir");
        fs::create_dir(&hidden_sub_dir_path).unwrap();
        let file4_path = hidden_sub_dir_path.join("test4.md");  
        let mut file4 = File::create(&file4_path).unwrap();
        writeln!(file4, "fn main() {{ println!(\"# MARKDOWN! in File4\"); }}").unwrap();
    
        // Expected size should only include test1, test2, and test3 (excluding hidden files)
        let expected_size = expected_size + file3_path.metadata().unwrap().len();
        let calc_size = calculate_directory_size(&dir_path, &exclude, &include, false).unwrap();
        assert_eq!(calc_size, expected_size);
    
        // Exclude `test1.txt` and recalculate
        let exclude = ExcludeList::new(&dir_path, vec![PathBuf::from("test1.txt")]);
        let expected_size = file2_path.metadata().unwrap().len() + file3_path.metadata().unwrap().len();
        let calc_size = calculate_directory_size(&dir_path, &exclude, &include, false).unwrap();
        assert_eq!(calc_size, expected_size);
    
        // Create a `.gitignore` file to exclude `test2.rs`
        let gitignore_path = dir_path.join(".gitignore");
        let mut gitignore = File::create(&gitignore_path).unwrap();
        writeln!(gitignore, "test2.rs").unwrap();
    
        // Recalculate with .gitignore; only file3 should be included
        let calc_size = calculate_directory_size(&dir_path, &exclude, &include, false).unwrap();
        let expected_size = file3_path.metadata().unwrap().len();
        assert_eq!(calc_size, expected_size);
    
        // Test including a hidden file
        let include = IncludeList::new(&dir_path, vec![PathBuf::from(".hidden_file.txt")]);
        let calc_size = calculate_directory_size(&dir_path, &exclude, &include, true).unwrap();
        let expected_size = expected_size + hidden_file_path.metadata().unwrap().len();
        assert_eq!(calc_size, expected_size);
    
        // Test including a hidden directory
        let include = IncludeList::new(&dir_path, vec![PathBuf::from(".hidden_subdir")]);
        let calc_size = calculate_directory_size(&dir_path, &exclude, &include, true).unwrap();
        let expected_size = expected_size + file4_path.metadata().unwrap().len();
        assert_eq!(calc_size, expected_size);
    }
}