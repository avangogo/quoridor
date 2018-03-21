extern crate cairo;
use self::cairo::Context;
use self::cairo::*;
use types;
use types::Orientation::{Horizontal, Vertical};

// Graphical params
const X0 : f64 = 20.;
const Y0 : f64 = 20.;
const C : f64 = 50.;
const WALL_WIDTH : f64 = 3.;

// Controller
pub const THRESHOLD : f64 = 0.25;

pub fn required_size(board_size: usize) -> f64 {
    ((board_size) as f64) * C + 2. * X0
}

// Geometry
fn cell_coord(i: usize, j: usize) -> (f64, f64) {
    ( i as f64 * C + X0,
      j as f64 * C + Y0 )
}

fn cell_center(i: usize, j: usize) -> (f64, f64) {
    ( (i as f64 + 0.5) * C + X0,
      (j as f64 + 0.5) * C + Y0 )
}

pub fn normalize( (x, y) : (f64, f64) ) -> (f64, f64) {
    ( (x - X0)/C, (y - Y0)/C )
}    

pub fn cell(context: &Context, cell: types::Cell) {
    context.set_source_rgb(0.7,0.3,0.2);
    let (x, y) = cell_coord(cell.x, cell.y);
    context.rectangle(x+WALL_WIDTH, y+WALL_WIDTH,C-2.*WALL_WIDTH,C-2.*WALL_WIDTH);
    context.fill();
}

pub fn wall(context: &Context, wall: types::Wall, shadow: bool) {
    let alpha = if shadow { 0.5 } else { 1. };
    context.set_source_rgba(0.1, 0.1, 0.1, alpha);
        
    let (x, y) = cell_coord(wall.x + 1, wall.y + 1);
    let l = C-WALL_WIDTH-2.;
    match wall.orientation {
        Horizontal => {
            context.rectangle(x-l, y-WALL_WIDTH,
                              2.*l, 2.*WALL_WIDTH);
        }
        Vertical => {
            context.rectangle(x-WALL_WIDTH, y-l,
                              2.*WALL_WIDTH, 2.*l);
        }
    }
    context.fill();
}

pub fn pawn(context: &Context, cell: types::Cell, player: usize, shadow: bool) {
    let fi = player as f64;

    let (cx, cy) = cell_center( cell.x, cell.y);
    let grad = RadialGradient::new(cx, cy, 0., cx, cy, C);
    grad.add_color_stop_rgb(0.5, 0., 0., 0.);
    if shadow {
        grad.add_color_stop_rgba(0., 0.5, 0.5, 0.5, 0.5);
    }
    else {
        grad.add_color_stop_rgb(0., fi, 0., 1. - fi);
    }
    
    context.set_source(&grad);
    //    self::cell(context, cell);
    context.arc(cx, cy, C*0.3, 0., 2.*3.14);
    context.fill();
}

pub fn wall_count(context: &Context, board_size: usize, c0: usize, c1: usize){
    let n = board_size as f64;
    let cx = X0 + n*C/2.;
    let cy = Y0 + n*C + 15.;
    let grad = LinearGradient::new(cx-10., cy, cx+10., cy);
    grad.add_color_stop_rgb(0., 0., 0., 1.);
    grad.add_color_stop_rgb(0.4, 0., 0., 0.);
    grad.add_color_stop_rgb(0.6, 0., 0., 0.);
    grad.add_color_stop_rgb(1., 1., 0., 0.);
    context.set_source(&grad);
    context.set_font_size(14.);
    context.move_to(cx-22., cy);
    context.show_text(format!("{}  /  {}", c0, c1).as_str());
}
