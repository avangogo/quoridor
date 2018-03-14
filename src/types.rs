use std::ops::Add;
use std::cmp::min;
use std::fmt;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
pub struct Cell {
    pub x: usize,
    pub y: usize,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
pub struct Wall {
    pub x: usize,
    pub y: usize,
    pub orientation: Orientation,
}

pub struct Maze {
    horizontal: Vec<Vec<bool>>,
    vertical: Vec<Vec<bool>>,
    middle: Vec<Vec<bool>>
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
pub enum Move {
    MovePawn(Cell),
    BuildWall(Wall),
}

use self::Dir::*;
use self::Orientation::*;
use self::Move::*;

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MovePawn(cell) => write!(f, "Move {} {}", cell.x, cell.y),
            BuildWall(wall) => match wall.orientation {
                Horizontal => write!(f, "Wall {} {} H", wall.x, wall.y),
                Vertical   => write!(f, "Wall {} {} V", wall.x, wall.y),
            }
        }
    }
}


pub const ALL_DIR: [Dir; 4] = [ Up, Down, Left, Right ];

pub fn new_matrix<T>(n: usize, m: usize, a:T) -> Vec<Vec<T>>
    where T: Clone
{
    let mut res = Vec::new();
    for i in 0..n {
        res.push(Vec::new());
        for _ in 0..m {
            res[i].push(a.clone())
        }
    }
    res
}

impl Dir {
    fn to_coord(&self) -> (isize, isize){
        match *self {
            Up    => (0,  1),
            Down  => (0, -1),
            Left  => (-1, 0),
            Right => (1, 0),
        }
    }
}

impl Add<Dir> for Cell {
    type Output = (isize, isize);
    fn add(self, dir: Dir) -> (isize, isize) {
        let ( x, y ) = dir.to_coord();
        ( self.x as isize + x, self.y as isize + y )
    }
}

impl Maze {
    pub fn new(size: usize) -> Maze {
        Maze {
            horizontal: new_matrix(size, size-1, false),
            vertical: new_matrix(size-1, size, false),
            middle: new_matrix(size-1, size-1, false),
        }
    }
    
    pub fn can_build(&self, wall: Wall) -> bool {
        !(self.middle[wall.x][wall.y] ||
            match wall.orientation {
                Horizontal => {
                    self.horizontal[wall.x][wall.y]
                        || self.horizontal[wall.x+1][wall.y]
                }
                Vertical => {
                    self.vertical[wall.x][wall.y]
                        || self.vertical[wall.x][wall.y+1]
                }
            })
    }

    pub fn build(&mut self, wall: Wall){
        self.middle[wall.x][wall.y] = true;
        match wall.orientation {
            Horizontal => {
                self.horizontal[wall.x][wall.y] = true;
                self.horizontal[wall.x+1][wall.y] = true;
            }
            Vertical => {
                self.vertical[wall.x][wall.y] = true;
                self.vertical[wall.x][wall.y+1] = true;
            }
        }
    }
    

    pub fn unbuild(&mut self, wall: Wall){
        self.middle[wall.x][wall.y] = false;
        match wall.orientation {
            Horizontal => {
                self.horizontal[wall.x][wall.y] = false;
                self.horizontal[wall.x+1][wall.y] = false;
            }
            Vertical => {
                self.vertical[wall.x][wall.y] = false;
                self.vertical[wall.x][wall.y+1] = false;
            }
        }
    }
    
    pub fn adjacent(&self, a: Cell, b: Cell) -> bool {
        if a.x == b.x && (a.y == b.y+1 || a.y+1 == b.y) {
            return !self.horizontal[a.x][min(a.y, b.y)]
        }
        if a.y == b.y && (a.x == b.x+1 || a.x+1 == b.x) {
            return !self.vertical[min(a.x, b.x)][a.y]
        }
        false
    }
}
