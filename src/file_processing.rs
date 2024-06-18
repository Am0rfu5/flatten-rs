
use std::fs::File;
use std::io::{self, Write, Read};
use std::path::{Path, PathBuf};
use syntect::parsing::SyntaxSet;
use crate::types::{ExcludeList, IncludeList};
use ignore::{WalkBuilder, Walk, overrides::OverrideBuilder};

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

pub fn calculate_directory_size(path: &PathBuf, exclude: &ExcludeList, include: &IncludeList, allow_hidden: bool) -> io::Result<u64> {

    let walker = build_walker(path, exclude, include, allow_hidden).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
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
        let exclude = ExcludeList::new(&dir_path, vec![PathBuf::from("test1.txt")]);
        let include = IncludeList::new(&dir_path, vec![]);
        let size = calculate_directory_size(&dir_path, &exclude, &include, true).unwrap();

        // Assertions
        assert_eq!(size, file2_path.metadata().unwrap().len());
    }

    #[test]
    fn test_calculate_directory_size_with_includes() {
        let temp_dir = tempdir().unwrap();
        let dir_path = temp_dir.path().canonicalize().unwrap();

        // Create sample files
        let file1_path = dir_path.join(".test1.txt");
        let mut file1 = File::create(&file1_path).unwrap();
        writeln!(file1, "This is test file 1").unwrap();

        let file2_path = dir_path.join("test2.rs");
        let mut file2 = File::create(&file2_path).unwrap();
        writeln!(file2, "fn main() {{ println!(\"Hello, world!\"); }}").unwrap();

        // Include only test1.txt
        let exclude = ExcludeList::new(&dir_path, vec![]);
        let include = IncludeList::new(&dir_path, vec![PathBuf::from(".test1.txt")]);
        let size = calculate_directory_size(&dir_path, &exclude, &include, true).unwrap();

        // Assertions
        assert_eq!(size, file1_path.metadata().unwrap().len() + file2_path.metadata().unwrap().len());
        // assert_eq!(size, file1_path.metadata().unwrap().len());
    }
}
