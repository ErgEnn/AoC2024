use std::collections::{HashMap, HashSet};
use std::time::Instant;
use util::grid::{Coordinate, Direction, Grid, LocAndDir, Projection};

fn main() {
    let input = include_str!("../input.txt").lines().map(str::to_string).collect::<Vec<String>>();

    let grid = Grid::from(&input, Direction::SE);

    first_problem(&grid);

    second_problem(&input);
}

fn include_str(_: &str) -> &'static str {
"...........#.....#......
...................#....
...#.....##.............
......................#.
..................#.....
..#.....................
....................#...
........................
.#........^.............
..........#..........#..
..#.....#..........#....
........#.....#..#......"
}

fn first_problem(map: &Grid) {
    let now = Instant::now();
    let mut guard_proj = map.iter().find(|proj| {proj.tile.unwrap().value == '^'}).unwrap();
    let mut dir = Direction::N;
    let mut distinct_coords: HashSet<&Coordinate> = HashSet::new();
    distinct_coords.insert(&guard_proj.tile.unwrap().coord);
    loop {
        let next_proj = guard_proj.step(&dir);
        match next_proj.tile {
            None => { break; }
            Some(tile) => {
                match tile.value {
                    '#' => {
                        dir = dir.rotate_cw_90()
                    }
                    '.' | '^' => {
                        let coord = &tile.coord;
                        distinct_coords.insert(coord);
                        guard_proj = next_proj;
                    }
                    _ => panic!()
                }
            }
        }
        /*
        map.print(|x| {
            if next_proj.tile.map_or(false, |t| t.coord == x.coord){
                return match dir {
                    Direction::N => {'^'}
                    Direction::E => {'>'}
                    Direction::S => {'V'}
                    Direction::W => {'<'}
                    _ => panic!()
                }
            }
            if distinct_coords.contains(&&x.coord) {
                return 'X';
            }
            x.value
        });
        println!()
        */
    }
    let result = distinct_coords.len();

    println!("Answer 1: {} ({:.2?})", result, now.elapsed());
}

fn walk(possible_obj: &Projection, mut dir: Direction, hit_obstacles: &HashSet<(&Coordinate,Direction)>) -> bool {
    dir = dir.rotate_cw_90();
    let mut guard_proj = possible_obj.clone();
    loop {
        let next_proj = guard_proj.step(&dir);
        match next_proj.tile {
            None => { break; }
            Some(tile) => {
                let coord = &tile.coord;

                match tile.value {
                    '#' => {
                        if(hit_obstacles.contains(&(coord,dir))){
                            return true;
                        }
                        dir = dir.rotate_cw_90();
                    }
                    '.' | '^' => {

                        guard_proj = next_proj;
                    }
                    _ => panic!()
                }
            }
        }
    }
    false
}


fn walk2(
    input: &Vec<String>,
    mut pos: Coordinate,
    mut dir: Direction,
    x_objs: &HashMap<i64,Vec<i64>>,
    y_objs: &HashMap<i64,Vec<i64>>,
    additional_obstacle: Option<Coordinate>,
    ) -> Option<Vec<LocAndDir>>{

    let mut collided_obstacles = HashSet::new();
    let mut visited: Vec<LocAndDir> = Vec::new();
    loop{
        //println!("{:?} {:?}", pos, dir);
        match dir {
            Direction::N => {
                match lt_util(x_objs, pos.x, additional_obstacle.filter(|a| a.x==pos.x).map(|a| a.y), pos.y) {
                    None => {break;}
                    Some(obstacle_y) => {
                        if collided_obstacles.contains(&LocAndDir{coord: Coordinate{x: pos.x, y:obstacle_y, system: Direction::SE}, dir}) {
                            return None;
                        }
                        (obstacle_y+1..pos.y+1).rev().for_each(|y| { visited.push(LocAndDir::new(pos.x, y,dir)); });
                        collided_obstacles.insert(LocAndDir{coord: Coordinate{x: pos.x, y:obstacle_y, system: Direction::SE}, dir});
                        pos = Coordinate{x: pos.x, y: obstacle_y+1, system: Direction::SE};
                        dir = dir.rotate_cw_90();
                    }
                }
            }
            Direction::E => {
                match gt_util(y_objs, pos.y, additional_obstacle.filter(|a| a.y==pos.y).map(|a| a.x), pos.x) {
                    None => {break;}
                    Some(obstacle_x) => {
                        if collided_obstacles.contains(&LocAndDir{coord: Coordinate{x: obstacle_x, y:pos.y, system: Direction::SE}, dir}) {
                            return None;
                        }
                        (pos.x..obstacle_x).for_each(|x| { visited.push(LocAndDir::new(x, pos.y ,dir)); });
                        collided_obstacles.insert(LocAndDir{coord: Coordinate{x: obstacle_x, y:pos.y, system: Direction::SE}, dir});
                        pos = Coordinate{x: obstacle_x-1, y: pos.y, system: Direction::SE};
                        dir = dir.rotate_cw_90();
                    }
                }
            }
            Direction::S => {
                match gt_util(x_objs, pos.x, additional_obstacle.filter(|a| a.x==pos.x).map(|a| a.y), pos.y) {
                    None => {break;}
                    Some(obstacle_y) => {
                        if collided_obstacles.contains(&LocAndDir{coord: Coordinate{x: pos.x, y:obstacle_y, system: Direction::SE}, dir}) {
                            return None;
                        }
                        (pos.y..obstacle_y).for_each(|y| { visited.push(LocAndDir::new(pos.x, y,dir)); });
                        collided_obstacles.insert(LocAndDir{coord: Coordinate{x: pos.x, y:obstacle_y, system: Direction::SE}, dir});
                        pos = Coordinate{x: pos.x, y: obstacle_y-1, system: Direction::SE};
                        dir = dir.rotate_cw_90();
                    }
                }
            }
            Direction::W => {
                match lt_util(y_objs, pos.y, additional_obstacle.filter(|a| a.y==pos.y).map(|a| a.x), pos.x) {
                    None => {break;}
                    Some(obstacle_x) => {
                        if collided_obstacles.contains(&LocAndDir{coord: Coordinate{x: obstacle_x, y:pos.y, system: Direction::SE}, dir}) {
                            return None;
                        }
                        (obstacle_x+1..pos.x+1).rev().for_each(|x| { visited.push(LocAndDir::new(x, pos.y ,dir)); });
                        collided_obstacles.insert(LocAndDir{coord: Coordinate{x: obstacle_x, y:pos.y, system: Direction::SE}, dir});
                        pos = Coordinate{x: obstacle_x+1, y: pos.y, system: Direction::SE};
                        dir = dir.rotate_cw_90();
                    }
                }
            }
            _ => panic!()
        }
    }

    let width = input.len() as i64;
    let height = input[0].len() as i64;


    match dir {
        Direction::N => {
            (0..pos.y+1).rev().for_each(|y| { visited.push(LocAndDir::new(pos.x, y,dir)); });
        }
        Direction::E => {
            (pos.x..width).for_each(|x| { visited.push(LocAndDir::new(x, pos.y ,dir)); });
        }
        Direction::S => {
            (pos.y..height).for_each(|y| { visited.push(LocAndDir::new(pos.x, y,dir)); });
        }
        Direction::W => {
            (0..pos.x+1).rev().for_each(|x| { visited.push(LocAndDir::new(x, pos.y ,dir)); });
        }
        _ => panic!()
    }

    Some(visited)

}
fn second_problem(input: &Vec<String>){
    let now = Instant::now();
    let mut x_objs: HashMap<i64,Vec<i64>> = HashMap::new();
    let mut y_objs: HashMap<i64,Vec<i64>> = HashMap::new();
    let mut pos = Coordinate{x: 0, y: 0, system: Direction::SE};
    let mut dir = Direction::N;

    for (y, line) in input.iter().enumerate() {
        for (x,chr) in line.chars().enumerate() {
            if chr == '#' {
                y_objs.entry(y as i64).or_insert(Vec::new()).push(x as i64);
                x_objs.entry(x as i64).or_insert(Vec::new()).push(y as i64);
                continue
            }

            if chr == '^' {
                pos = Coordinate{x: x as i64,y: y as i64, system: Direction::SE};
            }
        }
    }

    let mut visited_tiles: Vec<LocAndDir> = walk2(input, pos, dir, &x_objs, &y_objs, None).unwrap();
    let mut unique_tiles: HashSet<&Coordinate> = HashSet::new();
    let visited_tiles_total = visited_tiles.iter().filter_map(|x| {
        if unique_tiles.contains(&x.coord) {
            return None;
        }
        unique_tiles.insert(&x.coord);
        Some(&x.coord)
    }).count();


    let mut iter = visited_tiles.iter();
    let mut prev = iter.next().unwrap();
    let mut new_obstacles: HashSet<Coordinate> = HashSet::new();
    let mut tried_obstacles: HashSet<Coordinate> = HashSet::new();
    tried_obstacles.insert(prev.coord);
    for visited_tile in iter {
        if !tried_obstacles.contains(&visited_tile.coord) {
            //println!("try obstacle at {:?}", visited_tile);
            match walk2(input, prev.coord, prev.dir, &x_objs, &y_objs, Some(visited_tile.coord)) {
                None => {
                    new_obstacles.insert(visited_tile.coord);
                },
                Some(_) => {}
            }
        }
        tried_obstacles.insert(visited_tile.coord);
        prev = visited_tile
    }


/*
    for (y,line) in input.iter().enumerate() {
        for (x,ch) in line.chars().enumerate() {
            if new_obstacles.contains(&Coordinate::new(x as i64,y as i64)) {
                print!("O")
            }else {
                print!("{}", ch)
            }
        }
        println!()
    }*/
/*
    for (y,line) in input.iter().enumerate() {
        for (x,ch) in line.chars().enumerate() {
            if visited_tiles.contains(&Coordinate{x: x as i64, y: y as i64, system: Direction::SE}) {
                print!("X")
            }else {
                print!("{}", ch)
            }
        }
        println!()
    }
    */
    println!("Answer 2: {} {} ({:.2?})", new_obstacles.len(), visited_tiles_total, now.elapsed());
    //println!("New obstacles: {:?}", new_obstacles);
}

fn lt_util(objs: &HashMap<i64,Vec<i64>>, key: i64, additional: Option<i64>, needle: i64) -> Option<i64> {
    let vec: &Vec<i64> = match objs.get(&key) {
        None => { &Vec::new()},
        Some(x) => {x}
    };
    let mut iter = vec.iter();
    let mut prev: Option<i64>= None;
    loop {
        let list_val_opt = iter.next();
        match list_val_opt {
            None => {
                match additional {
                    None => {
                        return prev;
                    }
                    Some(add) => {
                        if add < needle {
                            match prev {
                                None => {return additional}
                                Some(prev_val) => { if add > prev_val {
                                    return additional;
                                }}
                            }
                        }
                        return prev;
                    }
                }
            }
            Some(list_val) => {
                if *list_val > needle{
                    match additional {
                        None => { return prev }
                        Some(add) => {
                            if add < needle{
                                match prev {
                                    None => {return additional}
                                    Some(prev_val) => {
                                        if add > prev_val{
                                            return additional;
                                        }
                                        return prev
                                    }
                                };
                            }
                            return prev
                        }
                    }
                }
                prev = Some(*list_val);
            }
        }
    }
}

fn gt_util(objs: &HashMap<i64,Vec<i64>>, key: i64, additional: Option<i64>, needle: i64) -> Option<i64> {
    let vec: &Vec<i64> = match objs.get(&key) {
        None => { &Vec::new()},
        Some(x) => {x}
    };
    let mut iter = vec.iter();
    loop {
        let list_val_opt = iter.next();
        match list_val_opt {
            None => {
                match additional {
                    None => { return None; }
                    Some(add) => { if add > needle { return additional;} return None;}
                }
            }
            Some(list_val) => {
                if *list_val > needle{
                    match additional {
                        None => { return Some(*list_val); }
                        Some(add) => {
                            if add > needle && add < *list_val{
                                return additional;
                            }
                            return Some(*list_val);
                        }
                    }
                }
            }
        }
    }
}

trait Search{
    fn lt_search(&self, x:i64) -> Option<i64>;
    fn gt_search(&self, x:i64) -> Option<i64>;
}
impl Search for Vec<i64> {
    fn lt_search(&self, target: i64) -> Option<i64> {
        let mut prev: Option<i64>= None;
        for v in self {
            if *v > target {
                return prev
            }
            prev = Some(*v);
        }
        prev
    }

    fn gt_search(&self, target: i64) -> Option<i64> {
        for v in self {
            if *v > target {
                return Some(*v)
            }
        }
        None
    }
}
