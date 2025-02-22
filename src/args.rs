use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "meow", about = "A CLI tool to concatenate diffrent files")]
pub struct Cli {

    #[arg(short = 'n', long="number")]
    pub show_line_numbers: bool,

    #[arg(short = 'b', long="blank")]
    pub no_blank_line_numbers: bool,

    ///List of input files
    pub input_files: Vec<PathBuf>,

}
