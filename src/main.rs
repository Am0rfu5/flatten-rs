use std::fs::{self, File};
use std::io::{self, Write, Read};
use std::path::PathBuf;
use structopt::StructOpt;
use walkdir::WalkDir;
use chrono::Local;

#[derive(StructOpt)]
struct Cli {
    /// The directory to flatten
    #[structopt(parse(from_os_str), default_value = ".")]
    directory: PathBuf,
    
    /// The output file
    #[structopt(parse(from_os_str), short, long)]
    output: Option<PathBuf>,
    
    /// Files to exclude
    #[structopt(short, long, parse(from_os_str))]
    exclude: Vec<PathBuf>,
}

fn main() -> io::Result<()> {
    let args = Cli::from_args();

    let output_file = match args.output {
        Some(path) => path,
        None => {
            let datetime = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
            let current_dir = args.directory.file_stem()
                .and_then(|os_str| os_str.to_str())
                .unwrap_or("root");
            PathBuf::from(format!("flatten-{}-{}.txt", current_dir, datetime))
        }
    };

    // Check directory size and prompt if it's too large
    let directory_size = calculate_directory_size(&args.directory)?;
    const SIZE_LIMIT: u64 = 10 * 1024 * 1024; // 10 MB
    if directory_size > SIZE_LIMIT {
        println!("Warning: The directory size is {} bytes. Do you want to continue? (y/n)", directory_size);
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        if input.trim().to_lowercase() != "y" {
            return Ok(());
        }
    }

    let mut output = File::create(&output_file)?;

    for entry in WalkDir::new(&args.directory)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        if path == output_file || args.exclude.contains(&path.to_path_buf()) {
            continue;
        }

        let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        let language = match extension {
            "rs" => "rust",
            "js" => "javascript",
            "py" => "python",
            "java" => "java",
            "html" => "html",
            "css" => "css",
            "sh" => "shell",
            _ => "text",
        };

        writeln!(output, "## {}", path.display())?;
        writeln!(output, "```{}", language)?;

        let mut file = File::open(path)?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)?;

        match String::from_utf8(contents) {
            Ok(text) => writeln!(output, "{}", text)?,
            Err(_) => writeln!(output, "<non-UTF-8 data>")?,
        }

        writeln!(output, "```")?;
        writeln!(output)?; // Add an empty line between files
    }

    Ok(())
}

fn calculate_directory_size(path: &PathBuf) -> io::Result<u64> {
    let mut size = 0;
    for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        size += entry.metadata()?.len();
    }
    Ok(size)
}
