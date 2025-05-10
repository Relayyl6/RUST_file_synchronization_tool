use clap::{arg, Parser};
use std::path::PathBuf;

// settig up clap 
#[derive(Parser, Debug)]
#[command(name = "file_sync", about = "A file synchronization tool", long_about = None)]
pub struct Cli {
    #[arg(long, short, value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// Source directory to sync from 
    #[arg(short, long, value_name = "DIR")] //
    pub source: Option<PathBuf>,

    /// Target directory to sync to
    /// If not provided, the source directory will be used as the target directory.
    #[arg(short, long, value_name = "DIR")]
    pub target: Option<PathBuf>,

    /// Perform a dry run without making changes
    #[arg(long, default_value_t = false)]
    pub dry_run: bool,

    /// Exclude files or directories matching the given pattern
    /// Can be specified multiple times to exclude multiple patterns.
    #[arg(short, long, global = true)]
    pub verbose: bool,
     
    /// Exclude files or directories matching the given pattern
    #[arg(long)]
    pub one_way: bool
}


