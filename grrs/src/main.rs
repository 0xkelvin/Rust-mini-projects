#![allow(unused)]

use clap::Parser;
use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;

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
    // println!("Hello, world!");

    // let pattern = std::env::args().nth(1).expect("no pattern given");
    // let path = std::env::args().nth(2).expect("no path given");

    // let args = Cli {
    //     pattern: pattern,
    //     path: std::path::PathBuf::from(path),
    // };

    let args = Cli::parse();
    //[improvement] use BufReader instead 
    // let content = std::fs::read_to_string(args.path)
    //     .expect("cound not read file");
    // // how to know what are the menthods available in 'content'
    // for line in content.lines() {
    //     if line.contains(&args.pattern) {
    //         println!("{}", line);
    //     }
    // }
    if let Ok(lines) = read_lines(args.path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line) = line {
                // todo!("need to improve nest 'if' statment");
                if line.contains(&args.pattern) {
                    println!("{}", line);
                }
            }
        }
    }

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}