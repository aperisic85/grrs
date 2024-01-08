use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    let file = File::open(&args.path).expect("cant open file");
    let b_reader = BufReader::new(file);

    for line in b_reader.lines() {
        if let Ok(xx) = line {
            if xx.contains(&args.pattern) {
                println!("{}", xx);
            } else {
                println!("Not found matching pattern");
            }
        } else {
            println!("there is no lines here");
        }
    }
}
