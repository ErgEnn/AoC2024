pub mod grid;

use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

pub fn split_to_int_tuples(line: &str) -> (i64, i64) {
    let mut items = line.split_whitespace();
    (items.next().unwrap().parse().unwrap(), items.next().unwrap().parse().unwrap())
}

pub fn split_to_str_tuples(line: &str) -> (&str, &str) {
    let mut items = line.split_whitespace();
    (items.next().unwrap(), items.next().unwrap())
}

pub fn read_lines(filename: &str) -> Vec<String> {
    let file_path = Path::new(filename);
    let file = File::open(&file_path);
    let buf = io::BufReader::new(file.unwrap());
    buf.lines().map(|l| l.expect("Could not parse line")).collect()
}

pub fn all_pairs<T>(vec: &Vec<T>) -> Vec<(&T, &T)> {
    let mut pairs = Vec::new();
    for i in 0..vec.len() {
        for j in i+1..vec.len() {
            pairs.push((&vec[i], &vec[j]));
        }
    }
    pairs
}


