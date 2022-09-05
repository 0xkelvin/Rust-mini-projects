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

    grrs::find_matches(args.path, &args.pattern, &mut std::io::stdout());
}



#[test]
fn find_a_match() {
    let mut result = Vec::new();
    grrs::find_matches(std::path::PathBuf::from("Cargo.toml"), "[dependencies]", &mut result);
    assert_eq!(result, b"[dependencies]\n");
}