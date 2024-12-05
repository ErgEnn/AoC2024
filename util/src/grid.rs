use std::slice::Iter;

#[derive(Debug)]
pub struct Grid{
    pub width: usize,
    pub height: usize,
    pub data: Vec<Tile>,
}

#[derive(Debug)]
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
    pub fn from(lines: Vec<String>, positive_dir: Direction) -> Self {
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
        if (x < 0 || x >= self.width as i64 || y < 0 || y >= self.height as i64) {
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

    pub fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self.data[y * self.width + x].value);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    pub fn iterator() -> Iter<'static, Direction> {
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
}

impl Projection<'_> {
    pub fn step(&self, dir: &Direction) -> Projection {
        let new_coord = self.tile.unwrap().coord.step(&dir);
        self.grid.get(new_coord.x, new_coord.y)
    }
}

