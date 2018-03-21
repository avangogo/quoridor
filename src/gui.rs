extern crate gtk;
extern crate gdk;
extern crate relm;
use self::gtk::{Inhibit,DrawingArea};
use self::gtk::WidgetExt;
use std::sync::{Arc,Mutex};
use std::rc::Rc;
use std::cell::RefCell;
use board::*;
use types::*;
use self::relm::*;
use self::relm::Widget;
use self::gdk::EventMask;

use types::Move::*;
use types::Orientation::*;
use draw;

#[derive(Msg)]
pub enum Msg {
    MouseOn( (f64, f64) ),
    SelectMove(Move),
    Update,
}

pub struct Model {
    board: Arc<Mutex<Board>>,
    possible_move: Vec<Move>,
    selected_move: Option<Move>,
    size: usize,
}

impl Model {

    fn new(board: Arc<Mutex<Board>>) -> Self {
        let mut res = Model {
            board: board.clone(),
            selected_move: None,
            possible_move: Vec::new(),
            size: 0,
        };
        res.raw_update();
        res
    }
    
    fn raw_update(&mut self) {
        let mut board = self.board.lock().unwrap();
        self.possible_move = board.possible_move();
        self.size = board.size;
        self.selected_move = None;
    }

    fn coord_to_move( x: f64, y: f64 ) -> Option<Move> {
        if x < 0. || y < 0. {
            return None
        }
        let nx = x.round() as usize;
        let ny = y.round() as usize;
        let gapx = (x - x.round()).abs();
        let gapy = (y - y.round()).abs();
        if nx >= 1 && ny >= 1 {
            if gapy < draw::THRESHOLD {
                return Some(BuildWall(
                    Wall { x: nx-1, y: ny-1, orientation: Horizontal } ))
            }
            if gapx < draw::THRESHOLD {
                return Some(BuildWall(
                    Wall { x: nx-1, y: ny-1, orientation: Vertical } ))
            }
            
        }
        return Some( MovePawn( Cell {x: x.floor() as usize, y: y.floor() as usize} ))
    }    
}

pub struct BoardView {
    drawing_area: DrawingArea,
    model: Rc<RefCell<Model>>,
}

impl Update for BoardView {

    type Model = Model;
    type ModelParam = Arc<Mutex<Board>>;
    type Msg = Msg;

    fn model(_: &Relm<Self>, board: Self::ModelParam ) -> Self::Model {
        Model::new(board)
    }
    
    fn update(&mut self, event: Self::Msg){
        let mut model = self.model.borrow_mut();
        match event {
            Msg::SelectMove( _ ) => { () }
            Msg::MouseOn( (x, y) ) => {
                let mut new_selec = None;
                if let Some(new_move) = Model::coord_to_move(x, y) {
                    if model.possible_move.iter().any(|&x| x == new_move) {
                            new_selec = Some(new_move)
                        }
                }
                if model.selected_move != new_selec {
                    model.selected_move = new_selec;               
                    self.drawing_area.queue_draw();
                }
            },
            Msg::Update => {
                model.raw_update();
                self.drawing_area.queue_draw();
            },
        };
    }
}

impl Widget for BoardView {
    type Root = DrawingArea;
    
    fn root(&self) -> Self::Root {
        self.drawing_area.clone()
    }

    fn view(relm: &Relm<Self>, model0: Self::Model) -> Self {

        
        let model = Rc::new(RefCell::new(model0));
        let drawing_area = DrawingArea::new();
        
        drawing_area.add_events((EventMask::BUTTON_PRESS_MASK |
                                 EventMask::BUTTON_RELEASE_MASK |
                                 EventMask::POINTER_MOTION_MASK |
                                 EventMask::SCROLL_MASK).bits() as i32);        

        drawing_area.set_size_request(500, 500);

        connect!(relm, drawing_area,
                 connect_motion_notify_event(_,e),
                 return (
                     Msg::MouseOn(draw::normalize(e.get_position())),
                     Inhibit(false)));

        // draw
        let model_clone = model.clone();
        drawing_area.connect_draw(move |_, context| {
            let model = model_clone.borrow_mut();
            let board = model.board.lock().unwrap();
            
            context.set_source_rgb(0.6,0.2,0.08);
            context.paint();

            let n = board.size;
             
            for i in 0..n {
                for j in 0..n {
                    draw::cell(context, Cell {x: i, y: j} );
                }
            }
            for &wall in &board.walls {
                draw::wall(context, wall, false)
            }
            for i in 0..2 {
                draw::pawn(context, board.pawns[i], i, false);
            }
            
            if let Some(mov) = model.selected_move {
                match mov { 
                    MovePawn(cell) => {
                        draw::pawn(context, cell, board.active_player, true);
                    }
                    BuildWall(wall) => {
                        draw::wall(context, wall, true);
                    }
                }
            }
            Inhibit(false)   
        });
        
        drawing_area.show_all();

        BoardView {
            drawing_area: drawing_area,
            model: model,
        }
    }
}
