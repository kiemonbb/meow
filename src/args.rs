use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "meow", about = "A CLI tool to concatenate different files")]
pub struct Cli {

    ///Show line numbers for all lines.
    #[arg(short = 'n', long="number")]
    pub show_line_numbers: bool,

    ///Do not show line numbers for blank lines.
    #[arg(short = 'b', long="blank")]
    pub no_blank_line_numbers: bool,

    //Do not display error message if the program cannot find input file
    #[arg(short = 's', long="sus")]
    pub no_input_file_error: bool,

    ///List of input files
    pub input_files: Vec<PathBuf>,

}
