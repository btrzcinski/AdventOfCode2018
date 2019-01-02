use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::env::args;

struct Metadata {
    two_letter_count: u64,
    three_letter_count: u64,
}

fn open_input_file(file_name: &str) -> File {
    return File::open(&Path::new(file_name)).unwrap();
}

fn metadata_from_file(file: &File) -> Metadata {
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();
    let mut metadata = Metadata { two_letter_count: 0, three_letter_count: 0 };

    while reader.read_line(&mut buffer).unwrap_or_default() > 0 {
        let trimmed_line = buffer.trim();
        let char_freqs = trimmed_line.chars().fold(
            HashMap::new(), |mut m, c| {
                *m.entry(c).or_insert(0) += 1;
                m
            });
        let mut counts_for_two = false;
        let mut counts_for_three = false;
        for f in char_freqs.values() {
            match f {
                2 => counts_for_two = true,
                3 => counts_for_three = true,
                _ => (),
            }
        }
        if counts_for_two {
            //println!("{} counts for two", trimmed_line);
            metadata.two_letter_count += 1;
        }
        if counts_for_three {
            //println!("{} counts for three", trimmed_line);
            metadata.three_letter_count += 1;
        }
        buffer.clear();
    }

    return metadata;
}

fn checksum(m: &Metadata) -> u64 {
    m.two_letter_count * m.three_letter_count
}

fn main() {
    let file_name = args().nth(1).unwrap();
    let metadata = metadata_from_file(&open_input_file(&file_name));
    println!("Count of twos: {}", metadata.two_letter_count);
    println!("Count of threes: {}", metadata.three_letter_count);
    println!("Checksum: {}", checksum(&metadata));
}
