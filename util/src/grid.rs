use std::fmt::{Debug, Formatter};
use std::slice::Iter;

#[derive(Debug)]
pub struct Grid{
    pub width: usize,
    pub height: usize,
    pub data: Vec<Tile>,
}

#[derive(Clone, Copy, Eq, Hash)]
pub struct Coordinate {
    pub x: i64,
    pub y: i64,
    pub system: Direction,
}

#[derive(Debug)]
pub struct Tile {
    pub coord: Coordinate,
    pub value: char,
}

#[derive(Clone, Copy, Debug)]
pub struct Projection<'a> {
    pub tile: Option<&'a Tile>,
    pub grid: &'a Grid
}

pub struct GridIterator<'a> {
    index: usize,
    grid: &'a Grid
}

#[derive(Clone, Copy, Debug, Eq, Hash)]
pub struct LocAndDir {
    pub coord: Coordinate,
    pub dir: Direction,
}

impl<'a> Iterator for GridIterator<'a> {
    type Item = Projection<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.grid.data.len() {
            let proj = Projection{tile: Some(&self.grid.data[self.index]), grid: self.grid};
            self.index += 1;
            return Some(proj)
        }
        None
    }
}

impl Grid{
    pub fn from(lines: &Vec<String>, positive_dir: Direction) -> Self {
        let height = lines.len();
        let width = lines[0].len();
        let mut data = Vec::with_capacity(width * height);
        for y in 0..height {
            for x in 0..width {
                let coord = Coordinate{x: x as i64, y: y as i64, system: positive_dir};
                data.push(Tile{coord, value: lines[y].chars().nth(x).unwrap()});
            }
        }

        Grid{width, height, data}
    }

    pub fn get(&self, x: i64, y: i64) -> Projection {
        if x < 0 || x >= self.width as i64 || y < 0 || y >= self.height as i64 {
            return Projection{tile: None, grid: &self};
        }
        let idx = (self.width as i64 * y + x) as usize;
        let tile = &self.data[idx];
        let grid = self;
        Projection{tile: Some(tile), grid}
    }

    pub fn iter(&self) -> GridIterator {
        GridIterator {
            index: 0,
            grid: &self
        }
    }

    pub fn print<F>(&self, f: F) where F: Fn(&Tile)->char {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", f(&self.data[y * self.width + x]));
            }
            println!()
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW
}

impl Direction {
    pub fn iterator_8() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 8] = [
            Direction::N,
            Direction::NE,
            Direction::E,
            Direction::SE,
            Direction::S,
            Direction::SW,
            Direction::W,
            Direction::NW];
        DIRECTIONS.iter()
    }

    pub fn iterator_4() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 4] = [
            Direction::N,
            Direction::E,
            Direction::S,
            Direction::W];
        DIRECTIONS.iter()
    }

    pub fn opposite(&self) -> Direction {
        match self {
            Direction::N => {Direction::S}
            Direction::NE => {Direction::SW}
            Direction::E => {Direction::W}
            Direction::SE => {Direction::NW}
            Direction::S => {Direction::N}
            Direction::SW => {Direction::NE}
            Direction::W => {Direction::E}
            Direction::NW => {Direction::SE}
        }
    }

    pub fn rotate_cw_90(&self) -> Direction {
        match self {
            Direction::N => {Direction::E}
            Direction::NE => {Direction::SE}
            Direction::E => {Direction::S}
            Direction::SE => {Direction::SW}
            Direction::S => {Direction::W}
            Direction::SW => {Direction::NW}
            Direction::W => {Direction::N}
            Direction::NW => {Direction::NE}
        }
    }
}
impl Coordinate {
    pub fn step(&self, dir: &Direction) -> Coordinate {
        match self.system {
            Direction::SE => {
                match dir {
                    Direction::N => {Coordinate{x: self.x, y: self.y-1, system: self.system}},
                    Direction::NE => {Coordinate{x: self.x+1, y: self.y-1, system: self.system}},
                    Direction::E => {Coordinate{x: self.x+1, y: self.y, system: self.system}},
                    Direction::SE => {Coordinate{x: self.x+1, y: self.y+1, system: self.system}},
                    Direction::S => {Coordinate{x: self.x, y: self.y+1, system: self.system}},
                    Direction::SW => {Coordinate{x: self.x-1, y: self.y+1, system: self.system}},
                    Direction::W => {Coordinate{x: self.x-1, y: self.y, system: self.system}},
                    Direction::NW => {Coordinate{x: self.x-1, y: self.y-1, system: self.system}},
                }
            },
            _ => {panic!()}
        }
    }

    pub fn new(x: i64, y: i64) -> Self{
        Coordinate{x,y,system:Direction::SE}
    }
}

impl PartialEq<Self> for Coordinate {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Debug for Coordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl Projection<'_> {
    pub fn step(&self, dir: &Direction) -> Self {
        let new_coord = &self.tile.unwrap().coord.step(&dir);
        self.grid.get(new_coord.x, new_coord.y)
    }
}

impl PartialEq<Self> for LocAndDir {
    fn eq(&self, other: &Self) -> bool {
        self.coord == other.coord && self.dir == other.dir
    }
}

impl LocAndDir {
    pub fn new(x: i64, y: i64, dir: Direction) -> Self {
        LocAndDir{coord: Coordinate::new(x,y),dir}
    }
}