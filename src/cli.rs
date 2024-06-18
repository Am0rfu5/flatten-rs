use structopt::StructOpt;
use std::path::PathBuf;

#[derive(StructOpt)]
pub struct Cli {
    /// The directory to flatten
    #[structopt(parse(from_os_str), default_value = ".")]
    pub directory: PathBuf,

    /// The output file
    #[structopt(parse(from_os_str), short, long)]
    pub output: Option<PathBuf>,

    /// Files to exclude
    #[structopt(parse(from_os_str), short, long)]
    pub exclude: Vec<PathBuf>,

    /// Files to include
    #[structopt(parse(from_os_str), short, long)]
    pub include: Vec<PathBuf>,

    /// Allow hidden files
    #[structopt(short, long)]
    pub allow_hidden: bool,
}
