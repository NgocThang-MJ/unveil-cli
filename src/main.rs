#![allow(unused)]
use clap::Parser;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() -> std::io::Result<()> {
    let args = Cli::parse();
    let pattern = &args.pattern;
    let file_path = &args.path;

    let content = std::fs::read_to_string(file_path).expect("Could not read file");

    for line in content.lines() {
        if line.contains(pattern) {
            println!("{}", line);
        }
    }

    let f = std::fs::File::open(file_path)?;

    Ok(())
}
