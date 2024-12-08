use std::collections::{HashMap, HashSet};
use util::all_pairs;

fn main() {
    let lines = include_str!("../input.txt")
        .lines()
        .collect::<Vec<&str>>();

    first_problem(&lines);

    second_problem(&lines);
}

fn include_str(_: &str) -> &'static str {
    "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"
}

fn first_problem(lines: &Vec<&str>){
    let height = lines.len() as i64;
    let width = lines[0].len() as i64;
    let mut antennas: HashMap<char, Vec<(i64,i64)>> = HashMap::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '.' {
                continue;
            }
            antennas.entry(c).or_insert(Vec::new()).push((x as i64, y as i64));
        }
    }
    let mut antinodes: HashSet<(i64,i64)> = HashSet::new();
    for (_, locations) in antennas {
        let pairs = all_pairs::<(i64,i64)>(&locations);
        for pair in pairs {
            let dx = pair.0.0 - pair.1.0;
            let dy = pair.0.1 - pair.1.1;
            let possible_antinodes = vec![(pair.0.0+dx,pair.0.1+dy),(pair.0.0-dx,pair.0.1-dy),(pair.1.0+dx,pair.1.1+dy),(pair.1.0-dx,pair.1.1-dy)];
            for possible_antinode in possible_antinodes {
                if possible_antinode == *pair.0 || possible_antinode == *pair.1 {
                    continue;
                }
                if possible_antinode.0 < 0
                    || possible_antinode.0 >= width
                    || possible_antinode.1 < 0
                    || possible_antinode.1 >= height {
                    continue;
                }
                antinodes.insert(possible_antinode);
            }
        }
    }

/*
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if antinodes.contains(&(x as i64, y as i64)) {
                print!("#")
            }else {
                print!("{}", c);
            }
        }
        println!()
    }
    println!("w:{} h:{} n:{:?}", width, height, antinodes);

 */
    println!("Answer 1: {}", antinodes.len());
}

fn second_problem(lines: &Vec<&str>){
    let height = lines.len() as i64;
    let width = lines[0].len() as i64;
    let mut antennas: HashMap<char, Vec<(i64,i64)>> = HashMap::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '.' {
                continue;
            }
            antennas.entry(c).or_insert(Vec::new()).push((x as i64, y as i64));
        }
    }
    let mut antinodes: HashSet<(i64,i64)> = HashSet::new();
    for (_, locations) in antennas {
        let pairs = all_pairs::<(i64,i64)>(&locations);
        for pair in pairs {
            let dx = pair.0.0 - pair.1.0;
            let dy = pair.0.1 - pair.1.1;
            let mut mul = 1;
            loop {
                let x = pair.0.0 + dx*mul;
                let y = pair.0.1 + dy*mul;

                if x < 0 || x >= width || y < 0 || y >= height {
                    break;
                }
                antinodes.insert((x,y));
                mul+=1;
            }
            let mut mul = 1;
            loop {
                let x = pair.0.0 - dx*mul;
                let y = pair.0.1 - dy*mul;

                if x < 0 || x >= width || y < 0 || y >= height {
                    break;
                }
                antinodes.insert((x,y));
                mul+=1;
            }
            let mut mul = 1;
            loop {
                let x = pair.1.0 + dx*mul;
                let y = pair.1.1 + dy*mul;

                if x < 0 || x >= width || y < 0 || y >= height {
                    break;
                }
                antinodes.insert((x,y));
                mul+=1;
            }
            let mut mul = 1;
            loop {
                let x = pair.1.0 - dx*mul;
                let y = pair.1.1 - dy*mul;

                if x < 0 || x >= width || y < 0 || y >= height {
                    break;
                }
                antinodes.insert((x,y));
                mul+=1;
            }

        }
    }

    println!("Answer 2: {}", antinodes.len());
}
