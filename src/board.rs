use types::*;
//use types::Dir::*;
use types::Move::*;
use types::Orientation::*;
use std::collections::VecDeque;

pub struct Board {
    pub size: usize,
    pub walls: Vec<Wall>,
    pub pawns: [ Cell; 2 ],
    pub wall_left: [ usize; 2 ],
    pub active_player: usize, // 0 or 1
    maze: Maze,
}


impl Board {
    pub fn new(size: usize, wall_count: usize) -> Self {
        Board {
            size: size,
            walls: Vec::new(),
            pawns: [ Cell {x:size/2, y:0 }, Cell {x:size/2, y:size-1 } ],
            wall_left: [ wall_count, wall_count ],
            active_player: 0,
            maze: Maze::new(size),
        }
    }
    
    fn next_player(&mut self){
        self.active_player = 1 - self.active_player;
    }

    pub fn winner(&self) -> Option<usize> {
        for (i, c) in self.pawns.iter().enumerate() {
            if c.y == (1-i) * self.size {
                return Some(i)
            }
        }
        return None
    }
    
    fn exists(&self, x:isize, y:isize) -> bool {
        let size = self.size as isize;
        0 <= x && x < size && 0 <= y && y < size 
    }
    
    fn add(&self, c: Cell, dir: Dir) -> Option<Cell> {
        let (x, y) = c + dir;
        if self.exists(x, y) {
            Some( Cell { x: x as usize, y: y as usize } )
        }
        else {
            None
        }
    }

    fn reachable_from(&self, a: Cell) -> Vec<Cell> {
        let mut res = Vec::new();
        for &dir in &ALL_DIR {
            if let Some(b) = self.add(a, dir) {
                if self.maze.adjacent(a, b) {
                    res.push(b);
                }
            }
        }
        res
    }

    pub fn dijkstra<I>(&self, s: Cell, t: I) -> Option<Vec<Cell>>
        where I : IntoIterator<Item = Cell>
    {
        let mut grid = new_matrix(self.size, self.size, None);
        let mut queue = VecDeque::new();
        for c in t {
            grid[c.x][c.y] = Some( (0, c) );
            queue.push_back(c);
        }
        while let Some(c) = queue.pop_front() {
            let (d, _) = grid[c.x][c.y].unwrap();
            for new_c in self.reachable_from(c) {
                if grid[new_c.x][new_c.y] == None {
                    grid[new_c.x][new_c.y] = Some( (d+1, c) );
                    queue.push_back(new_c);
                }
            }
        }
        match grid[s.x][s.y] {
            None => None,
            Some(_) => {
                let mut path = Vec::new();
                let mut c = s;
                loop {
                    path.push(c);
                    let (_, new_c) = grid[c.x][c.y].unwrap();
                    if c == new_c { break; };
                    c = new_c;
                }
                Some(path)
            }
        }
    }

    
    // Inefficient and dirty
    fn non_bloking_wall(&mut self, w: Wall) -> bool {
        let n = self.size;
        for i in 0..2 {
            let row = (0..n).map(|k| Cell { x:k, y:(1-i)*(n - 1) } );
            self.maze.build(w);
            let path = self.dijkstra(self.pawns[i], row);
            self.maze.unbuild(w);
            if path == None {
                return false
            }}
        return true
    }

    pub fn possible_walls(&mut self) -> Vec<Wall> {
        let mut res = Vec::new();
        for &orientation in [Horizontal, Vertical].iter() {
            for x in 0..self.size-1 {
                for y in 0..self.size-1 {
                    let wall = Wall {x: x, y: y, orientation: orientation};
                    if self.maze.can_build(wall) && self.non_bloking_wall(wall) {
                        res.push(wall);
                    }
                }
            }
        }
        res
    }

    pub fn possible_movement(&self, player: usize) -> Vec<Cell> {
        self.reachable_from(self.pawns[player])
            .into_iter()
            .filter(|&x| self.is_empty(x))
            .collect()
    }

    pub fn possible_move(&mut self) -> Vec<Move> {
        let mut res = Vec::new();
        for c in self.possible_movement(self.active_player) {
            res.push(MovePawn(c));
        }
        if self.wall_left[self.active_player] > 0 {
            for wall in self.possible_walls() {
                res.push(BuildWall(wall));
            }
        }
        res
    }
    
    fn is_empty(&self, c: Cell) -> bool {
        self.pawns.iter().all(|&x| x != c)
    }

    pub fn apply_move(&mut self, mov: Move){
        match mov {
            MovePawn(cell) => {
                let cell_pawn = self.pawns[self.active_player];
                if !self.maze.adjacent(cell, cell_pawn) {
                    panic!("Impossible move: {:?} is not accessible from {:?}",
                           cell, cell_pawn);
                }
                if !self.is_empty(cell) {
                    panic!("Impossible move: {:?} is not empty", cell);
                }
                self.pawns[self.active_player] = cell;
            },
            BuildWall(wall) => {
                if self.wall_left[self.active_player] == 0 {
                    panic!("Impossible to build: No wall left.");
                }
                if !self.maze.can_build(wall) {
                    panic!("Impossible to build {:?}", wall);
                }
                self.wall_left[self.active_player] -= 1;
                self.maze.build(wall);
                self.walls.push(wall);
            },
        }
        self.next_player();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn it_works() {
        let mut board = Board::new(9, 10);
    }
}
