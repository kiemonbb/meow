use anyhow::Result;
use clap::Parser;
use args::Cli;

mod file_functions;
mod args; 

fn main() -> Result<()> {
    let args = Cli::parse();
    Ok(())
}
