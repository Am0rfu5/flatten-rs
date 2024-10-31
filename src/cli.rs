use structopt::StructOpt;
use std::path::PathBuf;

/// Defines command-line arguments for the `flatten` application.
///
/// `Cli` is parsed at runtime to determine the directory to process,
/// the output file, and various include/exclude options, which control
/// how `flatten` will perform directory flattening and file processing.
#[derive(StructOpt)]
pub struct Cli {
    /// The directory to flatten
    ///
    /// Defaults to the current directory if not specified. 
    #[structopt(parse(from_os_str), default_value = ".")]
    pub directory: PathBuf,
    
    /// The output file where the flattened content will be saved.
    ///
    /// If not specified, an output file with a timestamped filename will be created.
    #[structopt(parse(from_os_str), short, long)]
    pub output: Option<PathBuf>,

    /// Files or directories to exclude during flattening.
    ///
    /// These paths will be ignored in the final output.
    #[structopt(parse(from_os_str), short, long)]
    pub exclude: Vec<PathBuf>,

    /// Files or directories to include explicitly during flattening.
    ///
    /// Only the files specified in this list will be processed.
    #[structopt(parse(from_os_str), short, long)]
    pub include: Vec<PathBuf>,

    /// Allow hidden files to be included in the output.
    ///
    /// If this flag is set, hidden files (those starting with a dot) will also
    /// be processed.
    #[structopt(short, long)]
    pub allow_hidden: bool,
}
