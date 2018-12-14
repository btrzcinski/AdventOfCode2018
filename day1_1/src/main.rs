use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::env::args;

// Taken from https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/open.html
fn open_input_file(file_name: &str) -> File {
    return File::open(&Path::new(file_name)).unwrap();
}

fn sum_nums_from_file(file: &File) -> i64 {
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();
    let mut accumulator = 0;

    while reader.read_line(&mut buffer).unwrap_or_default() > 0 {
        let trimmed_line = buffer.trim();
        accumulator += match i64::from_str_radix(trimmed_line, 10) {
            Err(why) => panic!("Couldn't parse '{}': {}", trimmed_line, why),
            Ok(num) => num,
        };
        buffer.clear();
    }

    return accumulator;
}

fn main() {
    let file_name = args().nth(1).unwrap();
    println!("Sum: {}", sum_nums_from_file(&open_input_file(&file_name)));
}
