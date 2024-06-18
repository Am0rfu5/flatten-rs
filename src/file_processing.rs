// src/file_processing.rs
use std::fs::File;
use std::io::{self, Write, Read};
use std::path::PathBuf;
use syntect::parsing::SyntaxSet;
// use crate::types::{ExcludeList, IncludeList};
use ignore::WalkerBuilder;

// pub fn process_files(directory: &PathBuf, output_file: &PathBuf, exclude: &ExcludeList, include: &IncludeList) -> io::Result<()> {
pub fn process_files(directory: &PathBuf, output_file: &PathBuf) -> io::Result<()> {
    let mut output = File::create(output_file)?;
    let ss = SyntaxSet::load_defaults_newlines();

    for result in WalkerBuilder::new(directory) {
        match result {
            Ok(entry) => { 
                if entry.file_type().map_or(false, |ft| ft.is_file()) {
                    let path = entry.path();
                    // get the file extension
                    let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("txt");
                    let syntax = ss.find_syntax_by_extension(extension).unwrap_or_else(|| ss.find_syntax_plain_text());
                    // Write the file path to the output file
                    writeln!(output, "## {}", path.display())?;
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
                };
            },
            Err(err) => println!("ERROR: {}", err),
        }
    }
    Ok(())
}
// pub fn calculate_directory_size(path: &PathBuf, exclude: &ExcludeList, include: &IncludeList) -> io::Result<u64> {
pub fn calculate_directory_size(path: &PathBuf) -> io::Result<u64> {
    let mut size = 0;

    for result in WalkerBuilder::new("./") {
        // Each item yielded by the iterator is either a directory entry or an
        // error, so either print the path or the error.
        match result {
            Ok(entry) => { 
                println!("{}", entry.path().display());
                if entry.file_type().map_or(false, |ft| ft.is_file()) {
                    size += entry.metadata().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?.len();
                };
            },
            Err(err) => println!("ERROR: {}", err),
        }
    }

    Ok(size)
}
