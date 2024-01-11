use clap::Parser;

#[derive(Debug)]
struct CustomError(String);

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<(), CustomError> {

    let path = "test.txt";
    let content = std::fs::read_to_string(path)
        .map_err(|err| CustomError(format!("Error reading `{}` : {}", path, err)))?;
    println!("file content {:?}", content);

    Ok(())
}
