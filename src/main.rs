mod cli;
mod file_processing;
mod types;

use std::io;
use std::path::PathBuf;
use cli::Cli;
use structopt::StructOpt;
use chrono::Local;
// use types::{ExcludeList, IncludeList};

fn main() -> io::Result<()> {
    let args = Cli::from_args();
    let directory = args.directory;

    // let exclude = ExcludeList::new(&directory, args.exclude);
    // let include = IncludeList::new(&directory, args.include);

    // // Troubleshooting print statement
    // println!("Exclusion list:");
    // for path in &exclude.0 {
    //     println!("{}", path.display());
    // }

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

    // Check directory size and prompt if it's too large
    let directory_size = file_processing::calculate_directory_size(&directory)?;
    // let directory_size = file_processing::calculate_directory_size(&directory, &exclude, &include)?;
    const SIZE_LIMIT: u64 = 10 * 1024 * 1024; // 10 MB
    if directory_size > SIZE_LIMIT {
        println!("Warning: The directory size is {} bytes. Do you want to continue? (y/n)", directory_size);
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        if input.trim().to_lowercase() != "y" {
            return Ok(());
        }
    }

    file_processing::process_files(&directory, &output_file)?;

    Ok(())
}
