use clap::Parser;
use std::path::PathBuf;

// settig up clap 
#[derive(Parser, Debug)]
#[command(name = "filesync")]
#[command(about = "A file synchronization tool", long_about = None)]
pub Struct Cli {
    // Path to th econfiguration file
    // PathBuf is a type-safe way to handle filesystem paths in Rust.
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,
    #[arg(short, long, value_name = "DIR")] //
    pub source: Option<PathBuf>,
    #[arg(short, long, value_name = "DIR")]
    pub target: Option<PathBuf>,
    #[arg(short, long, global = true)]
    pub verbose: bool,
    #[arg(long)]
    pub one_way: bool
}


