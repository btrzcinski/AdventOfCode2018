use std::env::args;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

fn read_polymer_string(file_name: &str) -> io::Result<String> {
    let mut file = File::open(&Path::new(file_name))?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    let buffer = buffer.trim().to_string();
    Ok(buffer)
}


fn main() {
    let file_name = args().nth(1).unwrap();
    let polymer_str = read_polymer_string(&file_name).unwrap();
    println!("{}", polymer_str);
}
