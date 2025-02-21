use std::{fs::{self, File}, io::{BufRead, BufReader,Write}, path::PathBuf};

use anyhow::{Result,bail};


pub fn display_file(file_path: &PathBuf) -> Result<()> {
    let file = File::open(file_path)?;
    let content = BufReader::new(file);
    for line in content.lines() {
        match line {
            Ok(line_content) => println!("{}",line_content),
            Err(e) => bail!("Failed to read line {}",e),
        }
    }
    Ok(())
}

pub fn concatenate_file(input_paths: &[PathBuf], output_path: &PathBuf) -> Result<()> {
    let mut output_file = File::create(output_path)?;

    for input_path in input_paths {
        let input_file = File::open(input_path)?;
        let content = BufReader::new(input_file);
        for line in content.lines() {
            writeln!(output_file,"{}",line?)?;
        }
    }
    display_file(output_path)?;
    Ok(())
}
