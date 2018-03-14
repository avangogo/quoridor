extern crate rand;
use player::Player;
use board;
use board::Board;
use self::rand::{thread_rng,Rng};
use types::*;


pub struct TomRandom {
    player: usize,
    board: board::Board,
}

impl TomRandom {
    pub fn new(size: usize, wall_count: usize, starts: bool) -> Self {
        TomRandom {
            player: if starts { 0 } else { 1 },
            board: Board::new(size, wall_count),
        }
    }
}

impl Player for TomRandom {
    fn input(&mut self, m: Move) {
        assert!(self.player != self.board.active_player);
        self.board.apply_move(m);
    }

    fn output(&mut self) -> Option<Move> {
        assert!(self.player == self.board.active_player);
        let moves = self.board.possible_move();
        let m = *thread_rng().choose(&moves[..]).unwrap();
        self.board.apply_move(m);
        Some(m)
    }
}

// BobbyGreedy
