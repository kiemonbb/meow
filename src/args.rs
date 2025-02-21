use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "meow", about = "A CLI tool to concatenate diffrent files")]
pub struct Cli {
    ///List of input files
    pub input_files: Vec<PathBuf>,

    ///Action to perform
    pub action: String,

    ///Output file
    pub output_file: PathBuf,
}
