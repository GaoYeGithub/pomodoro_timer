use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Modify to change Work duration in minutes
    #[arg(short, long, default_value_t = 1)]
    pub work_duration: u64,

    /// Modify to change Break duration in minutes
    #[arg(short, long, default_value_t = 1)]
    pub break_duration: u64,

    #[arg(short, long)]
    pub directory: Option<PathBuf>,

    #[arg(short, long)]
    pub initial_snapshot: Option<PathBuf>,
}