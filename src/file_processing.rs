use std::fs::File;
use std::io::{self, Write, Read};
use std::path::PathBuf;
use syntect::parsing::SyntaxSet;
use crate::types::{ExcludeList, IncludeList};
use ignore::{WalkBuilder, Walk, overrides::OverrideBuilder};

/// Constructs a file walker that recursively traverses a directory with specified
/// include and exclude filters.
/// 
/// This function leverages `ignore::WalkBuilder` to create a walker that follows
/// `include` and `exclude` lists, allowing for fine-grained file selection while
/// respecting hidden files if specified.
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
/// 
/// # Errors
///
/// This function may return an error if the `OverrideBuilder` fails to parse
/// any patterns provided in the `include` or `exclude` lists.
fn build_walker(directory: &PathBuf, exclude: &ExcludeList, include: &IncludeList, allow_hidden: bool) -> Result<Walk, ignore::Error> {
    let mut overrides = OverrideBuilder::new(directory);

    // Add inclusion patterns to the override builder, which takes priority over exclusions
    for inc in &include.0 {
        let inc_pattern = format!("{}", inc.display());
        overrides.add(&inc_pattern)?;
    }

    // Add exclusion patterns to ignore specific paths during traversal
    for exc in &exclude.0 {
        let exc_pattern = format!("{}", exc.display());
        overrides.add(&exc_pattern)?;
    }

    let overrides = overrides.build()?;

    // Create the file walker, setting it to include or ignore hidden files based on `allow_hidden`
    let walker = WalkBuilder::new(directory)
        .overrides(overrides)
        .hidden(!allow_hidden)
        .build();

    Ok(walker)
}

/// Processes all files in the specified directory, applying inclusion and exclusion filters,
/// and writes the flattened content into the given output file.
/// 
/// The function walks through the directory, applying include/exclude patterns and respecting
/// hidden files if allowed. Each file’s content is appended to the output file, with syntax
/// highlighting based on file extension, making it suitable for syntax-aware output formats.
/// 
/// # Arguments
/// 
/// * `directory` - The directory to process.
/// * `output_file` - The path to the output file where flattened content will be saved.
/// * `exclude` - An `ExcludeList` of paths to ignore during processing.
/// * `include` - An `IncludeList` of paths to include explicitly, even if they would otherwise be excluded.
/// * `allow_hidden` - A boolean flag to control whether hidden files should be processed.
///
/// # Returns
///
/// * `Ok(())` on successful processing.
/// * `Err(io::Error)` if file I/O operations (read/write) encounter issues.
///
/// # Errors
///
/// An error is returned if the output file cannot be created, a file within
/// the directory fails to open, or there are issues reading file contents.
pub fn process_files(
    directory: &PathBuf, 
    output_file: &PathBuf, 
    exclude: &ExcludeList, 
    include: &IncludeList, 
    allow_hidden: bool
) -> io::Result<()> {
    
    // Create the output file or return an error if creation fails
    let mut output = File::create(output_file)?;
    let ss = SyntaxSet::load_defaults_newlines();
    
    // Build the file walker, handling errors in directory access or invalid paths.
    let walker = build_walker(directory, exclude, include, allow_hidden)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    // Traverse the directory using the walker
    for result in walker {
        // Handle walker entry errors (e.g., permission denied on certain files)
        let entry = result.map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        let path = entry.path().canonicalize().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        
        // Skip directories, as we only process individual files
        if entry.file_type().map_or(false, |ft| ft.is_file()) && path != output_file.canonicalize().map_err(|e| io::Error::new(io::ErrorKind::Other, e))? {
            let rel_path = path.strip_prefix(directory).unwrap_or(&path);
            let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("txt");
            let syntax = ss.find_syntax_by_extension(extension).unwrap_or_else(|| ss.find_syntax_plain_text());

            // Write fileheader and syntax type to the output file
            writeln!(output, "## {}", rel_path.display())?;
            writeln!(output, "```{}", syntax.name.to_lowercase())?;

            // Open the file and read its content, return an error if read fails
            let mut file = File::open(&path)?;
            let mut contents = Vec::new();
            file.read_to_end(&mut contents)?;

            // Write formatted output with syntax highlighting based on file extension
            // Errors here are critical, so they propagate up the stack
            match String::from_utf8(contents) {
                Ok(text) => writeln!(output, "{}", text)?,
                Err(_) => writeln!(output, "<non-UTF-8 data>")?,
            }

            writeln!(output, "```")?; // Close the syntax block
            writeln!(output)?; // Add an empty line between files
        }
    }

    Ok(())
}


/// Calculates the total size of all files in a directory, respecting inclusion and exclusion
/// lists and optionally counting hidden files.
/// 
/// This function iterates over files within the specified directory, applying filters
/// to sum up the sizes of only the files that match the given criteria, providing a useful
/// size estimate prior to flattening or to verify size limits.
/// 
/// # Arguments
///
/// * `directory` - The directory containing files to size up.
/// * `exclude` - An `ExcludeList` specifying files or directories to ignore.
/// * `include` - An `IncludeList` specifying files or directories to explicitly include.
/// * `allow_hidden` - If true, includes hidden files in the total size calculation.
///
/// # Returns
/// 
/// * `Ok(u64)` - The total size (in bytes) of files meeting the criteria.
/// * `Err(io::Error)` if a file fails to open or retrieve metadata.
///
/// # Errors
///
/// Errors may arise if a file cannot be accessed due to permissions or if
/// there is an I/O error while retrieving file metadata.
pub fn calculate_directory_size(
    directory: &PathBuf, 
    exclude: &ExcludeList, 
    include: &IncludeList, 
    allow_hidden: bool
) -> io::Result<u64> {

    // Initialize a walker for directory traversal, respecting `allow_hidden`, `include`, and `exclude` settings
    let walker = build_walker(directory, exclude, include, allow_hidden)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    let mut size = 0;
    
    // Traverse files, summing sizes of files matching inclusion criteria   
    for result in walker {
        let entry = result
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        
        // Only include files, ignoring directories
        if entry.file_type().map_or(false, |ft| ft.is_file()) {
            // Retrieve file metadata to add its size to the total count
            size += entry.metadata()
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?.len();
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
    use std::path::PathBuf;

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
    fn test_calculate_directory_size_basic() {
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
    fn test_calculate_directory_size_excludes() {
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
        let exclude = ExcludeList::new(&dir_path, vec![PathBuf::from("test1.txt")]);
        let include = IncludeList::new(&dir_path, vec![]);
        let size = calculate_directory_size(&dir_path, &exclude, &include, true).unwrap();

        // Assertions
        assert_eq!(size, file2_path.metadata().unwrap().len());
    }

    #[test]
    fn test_hidden_files_not_included_by_default() {
        let temp_dir = tempdir().unwrap();
        let dir_path = temp_dir.path().canonicalize().unwrap();

        // Create a visible file and a hidden file
        let file1_path = dir_path.join("test1.txt");
        let mut file1 = File::create(&file1_path).unwrap();
        writeln!(file1, "This is test file 1").unwrap();

        let hidden_file_path = dir_path.join(".hidden_file.txt");
        let mut hidden_file = File::create(&hidden_file_path).unwrap();
        writeln!(hidden_file, "This is a hidden file").unwrap();

        // Calculate size without allowing hidden files
        let exclude = ExcludeList::new(&dir_path, vec![]);
        let include = IncludeList::new(&dir_path, vec![]);
        let size = calculate_directory_size(&dir_path, &exclude, &include, false).unwrap();

        // Only the visible file size should be counted
        assert_eq!(size, file1_path.metadata().unwrap().len());
    }

}
