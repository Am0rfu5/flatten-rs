mod cli;
mod file_processing;
mod types;

use std::io;
use std::path::PathBuf;
use cli::Cli;
use structopt::StructOpt;
use chrono::Local;
use types::{ExcludeList, IncludeList};
use std::fs::canonicalize;

/// Main function for the `flatten` CLI tool, handling argument parsing, directory
/// size verification, and file processing based on user input.
/// 
/// This function parses command-line arguments to set up include/exclude lists
/// and file paths, verifies directory size against a set limit, and, if confirmed,
/// flattens the directory content into a single output file. Errors in processing
/// or size verification prompt user input for continuation.
/// 
/// # Returns
///
/// * `Ok(())` on successful file processing.
/// * `Err(io::Error)` if any error arises during initialization or file handling.
///
/// # Errors
///
/// This function may return errors if files fail to open, arguments are invalid,
/// or if user input fails after exceeding the size limit warning.
fn main() -> io::Result<()> {
    // Parse command-line arguments into structured options
    let args = Cli::from_args();
    let directory = canonicalize(&args.directory)?;

    // Create inclusion and exclusion lists based on CLI arguments
    let exclude = ExcludeList::new(&directory, args.exclude);
    let include = IncludeList::new(&directory, args.include);
    
    // Determine output file path, generating a timestamped default if not specified
    let output_file = match args.output {
        Some(path) => path,
        None => {
            let datetime = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
            let current_dir = directory.file_stem()
                .and_then(|os_str| os_str.to_str())
                .unwrap_or("root");
            PathBuf::from(format!("flatten-{}-{}.txt", current_dir, datetime))
        }
    };

    // Estimate directory size and confirm with the user if it exceeds a preset limit (10 MB)
    let directory_size = file_processing::calculate_directory_size(&directory, &exclude, &include, args.allow_hidden)?;
    const SIZE_LIMIT: u64 = 10 * 1024 * 1024; // 10 MB
    if directory_size > SIZE_LIMIT {
        println!("Warning: The directory size is {} bytes. Do you want to continue? (y/n)", directory_size);
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        if input.trim().to_lowercase() != "y" {
            return Ok(());
        }
    }

    // Perform file flattening and generate the output file
    file_processing::process_files(&directory, &output_file, &exclude, &include, args.allow_hidden)?;

    Ok(())
}
