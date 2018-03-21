extern crate futures;

use types::Move;
use std::io::*;
use std::ffi::OsStr;
use std::process::*;
use std::error::Error;
use std::result::Result;
//use errors::*;
use errors::PlayerError::*;

#[derive(Debug, Copy, Clone)]
pub struct GameParam {
    pub size: usize,
    pub walls: usize,
    pub starts: bool,
}

pub trait PlayerLauncher : Send
{
    fn name(&self) -> String;
    fn start(&self, GameParam) -> Box<Player>;
    fn box_clone(&self) -> Box<PlayerLauncher>;
}

type MyResult<A> = Result<A,Box<Error>>;


pub trait Player
{
    fn wait_for_output(&mut self) -> MyResult<Move>;
    fn input(&mut self, m: Move);
}

#[derive(Debug, Clone)]
pub struct ProgramLauncher {
    pub script: String,
}

pub struct ProgramPlayer {
    stdin: ChildStdin,
    buffer_out: BufReader<ChildStdout>,
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

impl PlayerLauncher for ProgramLauncher {
    fn name(&self) -> String {
        self.script.clone()
    }
    
    fn start(&self, param: GameParam) -> Box<Player> {
        // params
        let player = if param.starts { 0 } else { 1 };
        let args =
            String::from(format!("{} {} {}", param.size, param.walls, player));
        // script thread
        let p = new_process(self.script.clone(), args.clone());
        Box::new(
            ProgramPlayer {
                stdin: p.stdin.unwrap(),  
                buffer_out: BufReader::new(p.stdout.unwrap()),
            })
    }

    fn box_clone(&self) -> Box<PlayerLauncher> {
        Box::new(self.clone())
    }
}

impl Player for ProgramPlayer {
    
    fn input(&mut self, m: Move) {
        if let Err(e) = writeln!(self.stdin, "{}", m) {
            eprintln!("Impossible to send move \"{}\" to script: {}", m, e);
        }}

    fn wait_for_output(&mut self) -> MyResult<Move> {
        let mut buf = String::new();
        match self.buffer_out.read_line(&mut buf) {
            Ok(_) => {
                assert!(buf.pop() == Some('\n'));
                buf.parse().map_err(|e| From::from(ParseError(e, buf.clone())))
            },
            Err(e) => Err(From::from(e)),
        }}
}
