use std::error::Error;
use std::num::ParseIntError;
use std::fmt;
use types::*;

#[derive(Clone,Debug)]
pub enum ParseMoveError {
    BadFirstWord(String),
    EmptyLine,
    NotEnoughParams(usize,usize),
    ParseIntError(ParseIntError),
    InvalidOrientation(String),
}

#[derive(Clone,Debug)]
pub enum MoveError {
    OutOfGrid,
    BlockedByWall(Wall),    
    BlockPlayer(usize),
    OutOfWall,
    NonAdjacent(Cell,Cell),
    NonEmpty,
}

#[derive(Clone,Debug)]
pub enum PlayerError {
    ParseError(ParseMoveError, String),
    InvalidMove(MoveError, Move),
}

use self::ParseMoveError::*;
use self::MoveError::*;
use self::PlayerError::*;

// ==================
impl fmt::Display for ParseMoveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BadFirstWord(ref x) => write!(f, "Bad first word : \"{}\"", x),
            EmptyLine => write!(f, "Empty line"),
            NotEnoughParams(expected, n) =>
                write!(f, "{} parameters expected, {} found", expected, n),
            ParseIntError(_) => write!(f, "Integer expected"),
            InvalidOrientation(ref x) =>
                write!(f, "Invalid wall orientation: V or H, found \"{}\"", x),
        }}
}

impl Error for ParseMoveError {
    fn description(&self) -> &str {
        match *self {
            BadFirstWord(_) =>"Invalid first word",
            EmptyLine => "Empty line",
            NotEnoughParams(_,_) => "Bad number of words",
            ParseIntError(_) => "Integer expected",
            InvalidOrientation(_) => "Wall orientation must be H or V",
        }}

    fn cause(&self) -> Option<&Error> {
        match *self {
            ParseIntError(ref e) => Some(e),
            _ => None,
        }}
}


// ==================
impl fmt::Display for MoveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            OutOfGrid => write!(f, "Coordinates outside of grid"),
            BlockedByWall(w) => write!(f, "Move blocked by wall {}", w),    
            BlockPlayer(i) => write!(f, "Blocks Player {}", i),
            OutOfWall => write!(f, "No more wall"),
            NonAdjacent(c1,c2) => write!(f, "{} and {} are not adjacent", c1, c2),
            NonEmpty => write!(f, "Target cell is non empty"),
        }}
}

impl Error for MoveError {
    fn description(&self) -> &str {
        match *self {
            OutOfGrid => "Coordinates outside of grid",
            BlockedByWall(_) => "Move blocked by a wall",    
            BlockPlayer(_) => "This move will block a player",
            OutOfWall => "Player out of wall",
            NonAdjacent(_,_) => "Cannot move to non adajcent cell",
            NonEmpty => "Non empty cell",
        }}

    fn cause(&self) -> Option<&Error> {
        None
    }
}

// ==================
impl fmt::Display for PlayerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseError(ref e, ref line) =>
                write!(f, "Error while parsing \"{}\" {}", line, e),
            InvalidMove(ref e, mov) =>
                write!(f, "Forbiden move: \"{}\" {}", mov, e),
        }}
}

impl Error for PlayerError {
    fn description(&self) -> &str {
        match *self {
            ParseError(_, _) => "Parse error",
            InvalidMove(_, _) => "Invalid move",
        }}
    
    fn cause(&self) -> Option<&Error> {
        match *self {
            ParseError(ref e, _) => Some(e),
            InvalidMove(ref e, _) => Some(e)
        }}
}
