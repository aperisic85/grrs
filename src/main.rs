use anyhow::{Context, Result};
use clap::Parser;
use std::io::{self, Write};

#[derive(Debug)]
struct CustomError(String);

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    writeln!(handle, "foo: {}", 42)?;

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("Could not read file {}", args.path.display()))?;
    grrs::find_matches(&content, &args.pattern, &mut std::io::stdout());

    Ok(())
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    grrs::find_matches(
        "where are\nyou now baby\nblah Mercedes benz",
        "where",
        &mut result,
    );
    assert_eq!(result, b"where are\n"); //fn find_matches() returns whole line so wee need to compare whole line not patter only
}
