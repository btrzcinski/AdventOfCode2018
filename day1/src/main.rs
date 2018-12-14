use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::env::args;

// Taken from https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/open.html
fn open_input_file(file_name: &str) -> File {
    return File::open(&Path::new(file_name)).unwrap();
}

fn sum_nums(nums: &Vec<i64>) -> i64 {
    return nums.into_iter().sum();
}

fn all_nums_from_file(file: &File) -> Vec<i64> {
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();
    let mut nums = Vec::new();

    while reader.read_line(&mut buffer).unwrap_or_default() > 0 {
        let trimmed_line = buffer.trim();
        let num = match i64::from_str_radix(trimmed_line, 10) {
            Err(why) => panic!("Couldn't parse '{}': {}", trimmed_line, why),
            Ok(n) => n,
        };
        nums.push(num);
        buffer.clear();
    }

    return nums;
}

fn first_repeated_frequency(nums: &Vec<i64>) -> i64 {
    let mut accumulator = 0;
    let mut freqs_seen = HashSet::new();

    for num in nums.into_iter().cycle() {
        if freqs_seen.contains(&accumulator) {
            break;
        }
        freqs_seen.insert(accumulator);
        accumulator += num;
    }

    return accumulator;
}

fn main() {
    let file_name = args().nth(1).unwrap();
    let all_nums = all_nums_from_file(&open_input_file(&file_name));
    println!("Sum: {}", sum_nums(&all_nums));
    println!("First Repeated Freq: {}", first_repeated_frequency(&all_nums));
}
