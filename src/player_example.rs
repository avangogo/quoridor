extern crate rand;
extern crate futures;
use player::{Player,PlayerLauncher,GameParam};
use board;
use board::Board;
use self::rand::{thread_rng,Rng};
use types::*;
//use std::error;
//use errors::PlayerError;
use std::error::Error;

type MyResult<A> = Result<A,Box<Error>>;

#[derive(Debug, Copy, Clone)]
pub enum Examples {
    Tom
}

use self::Examples::*;

pub struct TomRandom {
    player: usize,
    board: board::Board,
}

impl PlayerLauncher for Examples {
    fn name(&self) -> String {
        match *self {
            Tom => String::from("Tom Random")
        }}
    
    fn start(&self, param: GameParam) -> Box<Player> {
        let player = if param.starts { 0 } else { 1 };
        let board =  Board::new(param.size, param.walls);
        match *self {
            Tom => Box::new(TomRandom { player: player, board: board, })
        }
    }

    fn box_clone(&self) -> Box<PlayerLauncher> {
        Box::new(self.clone())
    }
}

impl Player for TomRandom {
    fn wait_for_output(&mut self) -> MyResult<Move> {
        assert!(self.player == self.board.active_player);
        let moves = self.board.possible_move();
        let m = *thread_rng().choose(&moves[..]).unwrap();
        self.board.apply_move(m).unwrap();
        Ok(m)
    }

    fn input(&mut self, m: Move) {
        assert!(self.player != self.board.active_player);
        self.board.apply_move(m).unwrap();
    }
}

// BobbyGreedy
