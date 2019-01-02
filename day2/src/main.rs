use std::collections::HashMap;
use std::collections::HashSet;
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
    File::open(&Path::new(file_name)).unwrap()
}

fn ids_from_file(file: &File) -> Vec<String> {
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();
    let mut ids = Vec::new();

    while reader.read_line(&mut buffer).unwrap_or_default() > 0 {
        ids.push(buffer.trim().to_string());
        buffer.clear();
    }

    ids
}

fn metadata_from_file(ids: &Vec<String>) -> Metadata {
    let mut metadata = Metadata { two_letter_count: 0, three_letter_count: 0 };

    for id in ids {
        let char_freqs = id.chars().fold(
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
            metadata.two_letter_count += 1;
        }
        if counts_for_three {
            metadata.three_letter_count += 1;
        }
    }

    metadata
}

fn splice_index(id: &str, idx: usize) -> String {
    id.chars()
      .take(idx)
      .chain(
        id.chars()
          .skip(idx + 1))
      .collect()
}

fn common_letters_for_close_boxes(ids: &Vec<String>) -> Option<String> {
    // Assume that all ids are the same length.
    // Starting with index 0, remove a letter, then check for duplicates.
    // The duplicate entry (if found) is the set of common letters.

    let max_index = ids[0].len() - 1;
    for n in 0..=max_index {
        let spliced_ids = ids.iter().map(|id| splice_index(id, n));
        let mut spliced_id_set = HashSet::new();
        for id in spliced_ids {
            if spliced_id_set.contains(&id) {
                return Some(id.to_string());
            }
            spliced_id_set.insert(id);
        }
    }

    None
}

fn checksum(m: &Metadata) -> u64 {
    m.two_letter_count * m.three_letter_count
}

fn main() {
    let file_name = args().nth(1).unwrap();
    let ids = ids_from_file(&open_input_file(&file_name));
    let metadata = metadata_from_file(&ids);
    println!("Count of twos: {}", metadata.two_letter_count);
    println!("Count of threes: {}", metadata.three_letter_count);
    println!("Checksum: {}", checksum(&metadata));
    println!("Common letters of correct box IDs: {}", common_letters_for_close_boxes(&ids).unwrap());
}
