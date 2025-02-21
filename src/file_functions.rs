use std::{fs::{self, File}, io::{BufRead, BufReader, Read, Write}, path::PathBuf};

use anyhow::{Result,bail};

#[allow(dead_code)]
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

pub fn write_file<T: Read>(input: T) -> Result<()> {
    let content = BufReader::new(input);
    for line in content.lines() {
        let line_content = line?;
        println!("{}",line_content);
    }
    Ok(())
}
