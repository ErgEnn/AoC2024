use std::vec::IntoIter;
use util::grid::{Direction, Grid, Projection};

fn main() {
    let input = include_str!("../input.txt").lines().map(str::to_string).collect::<Vec<String>>();

    let grid = Grid::from(input, Direction::SE);

    first_problem(&grid);

    second_problem(&grid);
}

fn include_str(_: &str) -> &'static str {
    ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
.........."
}

fn first_problem(grid: &Grid) {
    let result: usize = grid.iter().map(|proj| {
        if proj.tile.unwrap().value == 'X' {
            return Direction::iterator().filter(|dir| {
                let itr = vec!['M','A','S'].into_iter();
                if(check(proj, dir, itr)){
                    //println!("{:?} {:?}", proj.tile.unwrap().coord, dir);
                    return true;
                }
                false
            }).count()
        }
        0
    }).sum();
    println!("Answer 1: {}", result);
}

fn check(proj: Projection, dir: &Direction, mut itr: IntoIter<char>)-> bool {
    let next_letter = match itr.next() {
        Some(letter) => letter,
        None => return true
    };
    let new_proj = proj.step(dir);
    match new_proj.tile {
        None => false,
        Some(tile) => {tile.value == next_letter && check(new_proj, dir, itr)}
    }
}

fn second_problem(grid: &Grid) {
    let result: usize = grid.iter().filter(|proj| {
        if proj.tile.unwrap().value == 'A' {
            let nw = proj.step(&Direction::NW).tile;
            let ne = proj.step(&Direction::NE).tile;
            let se = proj.step(&Direction::SE).tile;
            let sw = proj.step(&Direction::SW).tile;
            match (
                nw.map_or(' ', |t| t.value),
                se.map_or(' ', |t| t.value),
                ne.map_or(' ', |t| t.value),
                sw.map_or(' ', |t| t.value)) {
                ('M','S','M','S') => return true,
                ('S','M','M','S') => return true,
                ('S','M','S','M') => return true,
                ('M','S','S','M') => return true,
                _ => return false
            }
        }
        false
    }).count();
    println!("Answer 2: {}", result);
}
