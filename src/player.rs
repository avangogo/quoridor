use types::Move;
use types::Move::{MovePawn, BuildWall};
use types::Orientation::{Horizontal, Vertical};
use types::{Wall,Cell};
use std::io::*;
use std::result::Result;
use std::ffi::OsStr;
use std::process::*;

pub trait Player {
    fn input(&mut self, m: Move);
    
    fn output(&mut self) -> Option<Move>;
}


pub struct ProgramPlayer {
    active: bool,
    process: Child,
    buf_out: BufReader<ChildStdout>,
}

fn new_process<S>(name: S, args: String) -> Child
    where S: AsRef<OsStr>
{
    Command::new(name)
        .args(args.split(" "))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Command failed to start")
}


impl ProgramPlayer {
    pub fn new(script_name: &str, size: usize, wall_count:usize, first: bool) -> Self {
        let player = if first { 0 } else { 1 };
        let args = String::from(format!("{} {} {}", size, wall_count, player));
        let mut p = new_process(script_name, args);
        ProgramPlayer {
            active: first,
            buf_out: BufReader::new(p.stdout.take().unwrap()),
            process: p,
        }
    }
}

impl Player for ProgramPlayer {
    fn input(&mut self, m: Move) {
        assert!(!self.active);
        self.active = true;
        let input = self.process.stdin.as_mut().unwrap();
        write_move(input, m).expect("toto");
    }

    fn output(&mut self) -> Option<Move> {
        assert!(self.active);
        let mut buf = String::new();
        match self.buf_out.read_line(&mut buf) {
            Ok(0) => None,
            Ok(_) => {
                self.active = false;
                assert!(buf.pop() == Some('\n'));
                Some(read_move(buf)) },
            Err(_) => panic!("toto"),
        }
    }
}


// impl Player for FilePlayer {
//     fn input(&mut self, m: Move) {
//         assert!(!self.active);
//         self.active = true;
//         write_move(&mut self.cin, m);
//     }

//     fn output(&mut self) -> Option<Move> {
//         assert!(!self.active);
//         let mut buf = String::new();
//         match self.cout.read_line(&mut buf) {
//             Ok(0) => None,
//             Ok(_) => {
//                 self.active = false;
//                 Some(read_move(buf)) },
//             Err(_) => panic!("toto"),
//         }
//     }

// } 

pub fn write_move<T>(cout: &mut T, m: Move) -> Result<(),Error>
    where T: Write + Sized {
    match m {
        MovePawn(cell) => writeln!(cout, "Move {} {}", cell.x, cell.y),
        BuildWall(wall) => match wall.orientation {
            Horizontal => writeln!(cout, "Wall {} {} H", wall.x, wall.y),
            Vertical   => writeln!(cout, "Wall {} {} V", wall.x, wall.y),
        }
    }
}

// FIXME : rattrapper les erreurs
pub fn read_move(s: String) -> Move {
    let split = s[..].split(" ");
    let vec : Vec<_> = split.collect();
    if vec.len() < 3 { panic!("\"{}\" is too short to be a move", s) };
    let x : usize = vec[1].parse().unwrap();
    let y : usize = vec[2].parse().unwrap();
    match vec[0] {
        "Move" => {
            MovePawn( Cell {x: x, y: y} )
        }
        "Wall" => {
            if vec.len() < 4 { panic!("\"{}\" too short to be a Wall move", s) };
            let o = match vec[3] {
                "V" => Vertical,
                "H" => Horizontal,
                _   => panic!("\"{}\" not a valid Wall move", s),
            };
            BuildWall( Wall {x: x, y: y, orientation: o} )
        }
        _ => panic!("toto")
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn comp() {
//         let s = String::from("Move 1 2");
//         let m = read_move(s);
//         let mut t = String::new();
//         write_move(&mut t, m);
//         assert_eq!(s, t);
//     }
// }
