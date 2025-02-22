use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
    path::PathBuf,
};

use crate::args::Cli;
use anyhow::{bail, Result};

#[allow(dead_code)]
pub fn display_file(file_path: &PathBuf) -> Result<()> {
    let file = File::open(file_path)?;
    let content = BufReader::new(file);
    for line in content.lines() {
        match line {
            Ok(line_content) => println!("{}", line_content),
            Err(e) => bail!("Failed to read line {}", e),
        }
    }
    Ok(())
}

pub fn write_file<T: Read>(input: T, args: &Cli, line_counter: &mut usize) -> Result<()> {
    let reader = BufReader::new(input);

    for line in reader.lines() {
        let line_content = line?;
        let is_blank = line_content.trim().is_empty();

        match (args.show_line_numbers, args.no_blank_line_numbers, is_blank) {
            (true, true, true) => println!("   {}", line_content),
            (true, _, _) => {
                println!("{}   {}", line_counter, line_content);
                *line_counter += 1;
            }
            (false, _, _) => println!("{}", line_content),
        }
    }
    Ok(())
}
