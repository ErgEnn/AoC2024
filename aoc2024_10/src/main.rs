use std::collections::HashSet;
use util::grid::{Coordinate, Direction, Grid, Projection};

fn main() {
    let input = include_str!("../input.txt").lines().map(str::to_string).collect::<Vec<String>>();

    let grid = Grid::from(&input, Direction::SE);

    first_problem(&grid);

    second_problem(&grid);
}

fn include_str(_: &str) -> &'static str {
    "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"
}

fn walk(proj: Projection, prev_val: i32) -> Vec<Coordinate> {
    match proj.tile {
        None => { Vec::new() }
        Some(t) => {
            //println!("on tile {:?}, prev: {prev_val}", t);
            let val = t.value.to_digit(10).unwrap() as i32;
            if prev_val+1 != val {
                return Vec::new();
            }
            if val == 9 {
                return vec!(proj.tile.unwrap().coord);
            }
            Direction::iterator_4().flat_map(|dir| {
                let next_proj = proj.step(dir);
                let result = walk(next_proj, val);
                result
            }).collect()
        }
    }
}

fn first_problem(map: &Grid) {
    let mut result = 0;

    for proj in map.iter() {
        if proj.tile.unwrap().value != '0' {
            continue;
        }
        let found_ends = walk(proj,-1);
        let unique: HashSet<Coordinate> = HashSet::from_iter(found_ends.into_iter());
        result += unique.iter().count();
    }

    println!("Answer 1: {}", result);
}

fn second_problem(map: &Grid) {
    let mut result = 0;

    for proj in map.iter() {
        if proj.tile.unwrap().value != '0' {
            continue;
        }
        let found_ends = walk(proj,-1);
        result += found_ends.len();
    }

    println!("Answer 2: {}", result);
}
