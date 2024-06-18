use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Cli {
    /// The directory to flatten
    #[structopt(parse(from_os_str), default_value = "./")]
    pub directory: PathBuf,
    
    /// The output file
    #[structopt(parse(from_os_str), short, long)]
    pub output: Option<PathBuf>,
    
    /// Files to exclude
    #[structopt(short, long, parse(from_os_str))]
    pub exclude: Vec<PathBuf>,

    /// Files to include, overriding any exclusions
    #[structopt(long, parse(from_os_str))]
    pub include: Vec<PathBuf>,
}
