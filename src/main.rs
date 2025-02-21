use std::{fs::File, io::{stdin, BufReader}};

use anyhow::Result;
use args::Cli;
use clap::Parser;
use file_functions::write_file;

mod args;
mod file_functions;

fn main() -> Result<()> {
    let args = Cli::parse();
    if args.input_files.is_empty() {
        write_file(stdin())?;
    }
    else {
        for file_path in args.input_files {
            let file = File::open(file_path)?;
            let content = BufReader::new(file); 
            write_file(content)?;
        }
    }
    Ok(())
}
