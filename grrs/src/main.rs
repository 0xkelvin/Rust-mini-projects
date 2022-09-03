#![allow(unused)]

use clap::Parser;
use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;
use std::result;
use anyhow::{Context, Result};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}


fn main() {
    let args = Cli::parse();

    find_matches(args.path, &args.pattern);
}

fn find_matches(file_path: std::path::PathBuf, pattern: &str) {
    match read_lines(file_path) {
        Ok(lines) => {
            for line in lines {
                if let Ok(line) = line {
                    // todo!("need to improve nest 'if' statment");
                    if line.contains(pattern) {
                        println!("{}", line);
                    }
                }
            }
        },
        Err(error) => {eprintln!("Error:: {}", error);}
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let result = File::open(filename);
    let file = match result {
        Ok(file) => {file},
        Err(error) => {return Err(error.into());}
    };
    Ok(io::BufReader::new(file).lines())
}