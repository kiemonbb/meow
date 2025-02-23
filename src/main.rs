use std::{
    fs::File,
    io::{stdin, BufReader},
};

use anyhow::{bail, Context, Result};
use args::Cli;
use clap::Parser;
use file_functions::write_file;

mod args;
mod file_functions;

fn main() -> Result<()> {
    let args = Cli::parse();
    run(&args)?;
    Ok(())
}


pub fn run(args: &args::Cli) -> Result<()> {
    let mut line_counter = 1;

    if args.input_files.is_empty() {
        write_file(stdin(), args, &mut line_counter)?;
    } else {
        for file_path in &args.input_files {
            // Try to open the file. If it fails and the flag is set, skip to the next file.
            let file = match File::open(file_path) {
                Ok(file) => file,
                Err(_) if args.no_input_file_error => continue,
                Err(_) => bail!("Failed to open file: {}", file_path.display()),
            };

            let reader = BufReader::new(file);

            write_file(reader, args, &mut line_counter).with_context(|| {
                format!(
                    "Failed to read line: {} in file: {}",
                    line_counter,
                    file_path.display()
                )
            })?;
        }
    }

    Ok(())
}

