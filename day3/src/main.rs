#[macro_use] extern crate text_io;

//use std::collections::HashMap;
use std::collections::HashSet;
use std::default::Default;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::iter;
use std::iter::FromIterator;
use std::path::Path;
use std::env::args;

type ClaimId = u64;
type ClaimMap = Vec<Vec<Vec<ClaimId>>>;

struct Claim {
    id: ClaimId,
    left: usize,
    top: usize,
    width: usize,
    height: usize
}

impl Default for Claim {
    fn default() -> Claim {
        Claim {
            id: 0,
            left: 0,
            top: 0,
            width: 0,
            height: 0,
        }
    }
}

impl fmt::Display for Claim {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{} @ {},{}: {}x{}",
               self.id, self.left, self.top, self.width, self.height)
    }
}

fn open_input_file(file_name: &str) -> File {
    File::open(&Path::new(file_name)).unwrap()
}

fn claims_from_file(file: &File) -> Vec<Claim> {
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();
    let mut claims = Vec::new();

    while reader.read_line(&mut buffer).unwrap_or_default() > 0 {
        let mut c: Claim = Default::default();
        scan!(buffer.bytes() => "#{} @ {},{}: {}x{}",
              c.id, c.left, c.top, c.width, c.height);
        claims.push(c);
        buffer.clear();
    }

    claims
}

fn new_claim_map() -> ClaimMap {
    iter::repeat(
        iter::repeat(vec![]).take(1000).collect::<Vec<_>>())
        .take(1000).collect::<Vec<_>>()
}

fn claim_map_from_claims(claims: &Vec<Claim>) -> ClaimMap {
    let mut claim_map = new_claim_map();

    for c in claims {
        for col in c.left..(c.left + c.width) {
            for row in c.top..(c.top + c.height) {
                claim_map[col][row].push(c.id);
            }
        }
    }

    claim_map
}

fn overlapping_square_inches(claim_map: &ClaimMap) -> usize {
    claim_map.iter().flatten().filter(|cell| cell.len() > 1).map(|_| 1).sum()
}

fn non_overlapping_claim(max_claim_id: ClaimId, claim_map: &ClaimMap) -> Option<ClaimId> {
    let mut non_overlapping_claims: HashSet<ClaimId> =
        HashSet::from_iter(1..=max_claim_id);

    for overlapping_id in claim_map.iter().flatten()
                                   .filter(|cell| cell.len() > 1).flatten() {
        non_overlapping_claims.remove(overlapping_id);
    } 
    
    non_overlapping_claims.into_iter().nth(0)
}

fn main() {
    let file_name = args().nth(1).unwrap();
    let claims = claims_from_file(&open_input_file(&file_name));
    let claim_map = claim_map_from_claims(&claims);
    println!("First claim: {}", claims.first().unwrap());
    println!("0,862 in claim map: {:?}", claim_map[0][862]);
    println!("Overlapping square inches: {}",
             overlapping_square_inches(&claim_map));

    let max_claim_id = claims.iter().map(|c| c.id).max().unwrap();
    println!("Non-overlapping claim: {}",
             non_overlapping_claim(max_claim_id, &claim_map).unwrap());
}
