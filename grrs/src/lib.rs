
use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;

pub fn find_matches(file_path: std::path::PathBuf, pattern: &str, mut writer: impl std::io::Write) {
    match read_lines(file_path) {
        Ok(lines) => {
            for line in lines {
                if let Ok(line) = line {
                    // todo!("need to improve nest 'if' statment");
                    if line.contains(pattern) {
                        // println!("{}", line);
                        writeln!(writer,"{}", line).unwrap();

                    }
                }
            }
        },
        Err(error) => {eprintln!("Error:: {}", error);}
    }
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let result = File::open(filename);
    let file = match result {
        Ok(file) => {file},
        Err(error) => {return Err(error.into());}
    };
    Ok(io::BufReader::new(file).lines())
}