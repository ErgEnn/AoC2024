use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};

fn main() {
    let file_path = Path::new("input.txt");
    let file = File::open(&file_path);
    let buf = io::BufReader::new(file.unwrap());
    let left_right: (Vec<i32>, Vec<i32>) = buf
        .lines()
        .filter_map(|res| res.ok())
        .map(|res| split_to_tuples(&res))
        .unzip();
    first_problem(&left_right);

    second_problem(&left_right);
}

fn first_problem(left_right: &(Vec<i32>, Vec<i32>)){

    let (mut left, mut right) = left_right.clone();

    left.sort();
    right.sort();

    let sum: i32 = left.iter()
        .zip(right.iter())
        .map(|(l,r)| (l-r).abs())
        .sum();

    println!("Answer 1: {}", sum);
}

fn second_problem(left_right: &(Vec<i32>, Vec<i32>)){
    let (left, right) = left_right.clone();
    let mut counts = HashMap::new();
    for r in right {
        *counts.entry(r).or_insert(0) += 1;
    }
    let sum: i32 = left.iter().map(|l| l * counts.get(l).unwrap_or(&0)).sum();
    println!("Answer 2: {}", sum);
}

fn split_to_tuples(line: &str) -> (i32, i32) {
    let mut items = line.split_whitespace();
    (items.next().unwrap().parse().unwrap(), items.next().unwrap().parse().unwrap())
}
