use clap::Parser;
use anyhow::{Context, Result};

#[derive(Debug)]
struct CustomError(String);

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<()>{

    let path = "test.txt";
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Could not read file {}", path))?;
    println!("file content {:?}", content);

    Ok(())
}
